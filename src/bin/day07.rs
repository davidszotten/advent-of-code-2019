use aoc2019::{dispatch, Result};
use failure::{bail, err_msg, Error};
use permutohedron::LexicalPermutation;
use std::collections::VecDeque;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i32> for Mode {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        use Mode::*;

        Ok(match value {
            0 => Position,
            1 => Immediate,
            _ => bail!("Invalid op code"),
        })
    }
}

struct Modes {
    value: i32,
}

impl Iterator for Modes {
    type Item = Mode;

    fn next(&mut self) -> Option<Self::Item> {
        let mode = Mode::try_from(self.value % 10).ok();
        self.value /= 10;
        mode
    }
}

impl Modes {
    fn new(value: i32) -> Self {
        Modes { value }
    }

    fn get(&mut self) -> Result<Mode> {
        self.next().ok_or(err_msg("not enough modes"))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    Halt,
}

impl TryFrom<i32> for Op {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        use Op::*;

        let op_value = value % 100;
        let value = value / 100;
        let mut modes = Modes::new(value);

        let op = match op_value {
            1 => Add(modes.get()?, modes.get()?, modes.get()?),
            2 => Mul(modes.get()?, modes.get()?, modes.get()?),
            3 => Input(modes.get()?),
            4 => Output(modes.get()?),
            5 => JumpIfTrue(modes.get()?, modes.get()?),
            6 => JumpIfFalse(modes.get()?, modes.get()?),
            7 => LessThan(modes.get()?, modes.get()?, modes.get()?),
            8 => Equals(modes.get()?, modes.get()?, modes.get()?),
            99 => Halt,
            _ => bail!("Invalid op code"),
        };
        Ok(op)
    }
}

struct Cpu {
    pc: usize,
    program: Vec<i32>,
    input: VecDeque<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CpuState {
    Output(i32),
    NeedsInput,
    Halted,
}

impl Cpu {
    fn new(program: Vec<i32>) -> Self {
        Cpu {
            pc: 0,
            program,
            input: VecDeque::new(),
        }
    }

    fn from_str(program_str: &str) -> Self {
        let program: Vec<_> = program_str
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        Self::new(program)
    }

    fn get(&self, mode: Mode, source: i32) -> i32 {
        match mode {
            Mode::Immediate => source,
            Mode::Position => self.program[source as usize],
        }
    }

    fn set(&mut self, destination: i32, value: i32) {
        self.program[destination as usize] = value;
    }

    fn run(&mut self) -> Result<CpuState> {
        let state = loop {
            let op = Op::try_from(self.program[self.pc])?;
            use Op::*;
            match op {
                Add(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(c, self.get(mode1, a) + self.get(mode2, b));
                    self.pc += 4;
                }
                Mul(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(c, self.get(mode1, a) * self.get(mode2, b));
                    self.pc += 4;
                }
                Input(mode) => {
                    assert_eq!(mode, Mode::Position);
                    let a = self.program[self.pc + 1];
                    match self.input.pop_front() {
                        None => break CpuState::NeedsInput,
                        Some(value) => {
                            self.set(a, value);
                            self.pc += 2;
                        }
                    }
                }
                Output(mode) => {
                    let a = self.program[self.pc + 1];
                    let value = self.get(mode, a);
                    self.pc += 2;
                    break CpuState::Output(value);
                }
                JumpIfTrue(mode1, mode2) => {
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    if self.get(mode1, a) != 0 {
                        self.pc = self.get(mode2, b) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                JumpIfFalse(mode1, mode2) => {
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    if self.get(mode1, a) == 0 {
                        self.pc = self.get(mode2, b) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                LessThan(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(
                        c,
                        if self.get(mode1, a) < self.get(mode2, b) {
                            1
                        } else {
                            0
                        },
                    );
                    self.pc += 4;
                }
                Equals(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(
                        c,
                        if self.get(mode1, a) == self.get(mode2, b) {
                            1
                        } else {
                            0
                        },
                    );
                    self.pc += 4;
                }

                Halt => break CpuState::Halted,
            }
        };
        Ok(state)
    }
}

fn calculate(program_str: &str, input_values: &[i32]) -> Result<i32> {
    let mut cpu = Cpu::from_str(program_str);
    for input_value in input_values.iter() {
        cpu.input.push_back(*input_value);
    }
    match cpu.run()? {
        CpuState::Output(value) => Ok(value),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut phases = vec![0, 1, 2, 3, 4];
    let mut max_signal = 0;
    loop {
        let mut signal = 0;
        for &phase in phases.iter() {
            signal = calculate(input, &[phase, signal])?;
        }

        if signal > max_signal {
            max_signal = signal;
        }

        if !phases.next_permutation() {
            break;
        }
    }
    Ok(max_signal)
}

fn part2(input: &str) -> Result<i32> {
    let mut phases = vec![5, 6, 7, 8, 9];
    let mut max_signal = 0;

    loop {
        let mut cpus = vec![
            Cpu::from_str(input),
            Cpu::from_str(input),
            Cpu::from_str(input),
            Cpu::from_str(input),
            Cpu::from_str(input),
        ];

        for (index, &phase) in phases.iter().enumerate() {
            cpus[index].input.push_back(phase);
        }
        cpus[0].input.push_back(0);

        let mut running = [true; 5];
        for index in (0..5).cycle() {
            if !running.iter().any(|&r| r) {
                break;
            }
            let state = cpus[index].run()?;
            // dbg!(index, state);
            match state {
                CpuState::Halted => running[index] = false,
                CpuState::NeedsInput => {}
                CpuState::Output(value) => cpus[(index + 1) % 5].input.push_back(value),
            }
        }
        let signal = cpus[0].input.pop_front().expect("final output missing");

        if signal > max_signal {
            max_signal = signal;
        }

        if !phases.next_permutation() {
            break;
        }
    }
    Ok(max_signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op() -> Result<()> {
        assert_eq!(
            Op::try_from(1002)?,
            Op::Mul(Mode::Position, Mode::Immediate, Mode::Position)
        );
        Ok(())
    }

    #[test]
    fn test_part1a() -> Result<()> {
        assert_eq!(
            part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")?,
            43210
        );
        Ok(())
    }

    #[test]
    fn test_part1b() -> Result<()> {
        assert_eq!(
            part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")?,
            54321
        );
        Ok(())
    }

    #[test]
    fn test_part1c() -> Result<()> {
        assert_eq!(
            part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )?,
            65210
        );
        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<()> {
        assert_eq!(
            part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )?,
            139629729
        );
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )?,
            18216
        );
        Ok(())
    }
}
