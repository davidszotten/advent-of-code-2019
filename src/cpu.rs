use super::Result;
use failure::{bail, err_msg, Error};
use std::collections::VecDeque;
use std::convert::TryFrom;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuState {
    Output(i32),
    NeedsInput,
    Halted,
}

pub struct Cpu {
    pc: usize,
    program: Vec<i32>,
    input: VecDeque<i32>,
}

impl Cpu {
    fn new(program: Vec<i32>) -> Self {
        Cpu {
            pc: 0,
            program,
            input: VecDeque::new(),
        }
    }

    pub fn from_str(program_str: &str) -> Self {
        let program: Vec<_> = program_str
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        Self::new(program)
    }

    pub fn enqueue_input(&mut self, value: i32) {
        self.input.push_back(value);
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

    pub fn run(&mut self) -> Result<CpuState> {
        let state = loop {
            let op = Op::try_from(self.program[self.pc])?;
            // dbg!(self.pc, &op, &self.program);
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

pub fn read_memory(cpu: &Cpu, position: usize) -> i32 {
    cpu.program[position]
}

pub fn set_memory(cpu: &mut Cpu, position: usize, value: i32) {
    cpu.program[position] = value;
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
}
