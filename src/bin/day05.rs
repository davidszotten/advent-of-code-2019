use aoc2019::{dispatch, Result};
use failure::{bail, err_msg, Error};
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
    program: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl Cpu {
    fn new(program: Vec<i32>) -> Self {
        Cpu {
            program,
            input: vec![],
            output: vec![],
        }
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

    fn run(&mut self) -> Result<i32> {
        let mut pc = 0;
        loop {
            let op = Op::try_from(self.program[pc])?;
            use Op::*;
            match op {
                Add(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    let c = self.program[pc + 3];
                    self.set(c, self.get(mode1, a) + self.get(mode2, b));
                    pc += 4;
                }
                Mul(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    let c = self.program[pc + 3];
                    self.set(c, self.get(mode1, a) * self.get(mode2, b));
                    pc += 4;
                }
                Input(mode) => {
                    assert_eq!(mode, Mode::Position);
                    let a = self.program[pc + 1];
                    let value = self.input.pop().ok_or(err_msg("No input available"))?;
                    self.set(a, value);

                    pc += 2;
                }
                Output(mode) => {
                    let a = self.program[pc + 1];
                    let value = self.get(mode, a);
                    self.output.push(value);

                    pc += 2;
                }
                JumpIfTrue(mode1, mode2) => {
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    if self.get(mode1, a) != 0 {
                        pc = self.get(mode2, b) as usize;
                    } else {
                        pc += 3;
                    }
                }
                JumpIfFalse(mode1, mode2) => {
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    if self.get(mode1, a) == 0 {
                        pc = self.get(mode2, b) as usize;
                    } else {
                        pc += 3;
                    }
                }
                LessThan(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    let c = self.program[pc + 3];
                    self.set(
                        c,
                        if self.get(mode1, a) < self.get(mode2, b) {
                            1
                        } else {
                            0
                        },
                    );
                    pc += 4;
                }
                Equals(mode1, mode2, mode3) => {
                    assert_eq!(mode3, Mode::Position);
                    let a = self.program[pc + 1];
                    let b = self.program[pc + 2];
                    let c = self.program[pc + 3];
                    self.set(
                        c,
                        if self.get(mode1, a) == self.get(mode2, b) {
                            1
                        } else {
                            0
                        },
                    );
                    pc += 4;
                }

                Halt => break,
            }
        }
        let output = self.output.pop().ok_or(err_msg("no output"))?;
        assert_eq!(self.output.iter().all(|&x| x == 0), true);
        Ok(output)
    }
}

fn calculate(program_str: &str, input_value: i32) -> Result<i32> {
    let program: Vec<_> = program_str
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let mut cpu = Cpu::new(program);
    cpu.input.push(input_value);
    cpu.run()
}

fn part1(input: &str) -> Result<i32> {
    calculate(input, 1)
}

fn part2(input: &str) -> Result<i32> {
    calculate(input, 5)
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
    fn test_part1() -> Result<()> {
        // assert_eq!(part1("1002,4,3,4,33")?, 0);
        // assert_eq!(part1("1101,100,-1,4,0")?, 0);
        assert_eq!(calculate("3,9,8,9,10,9,4,9,99,-1,8", 8)?, 1);
        assert_eq!(calculate("3,9,8,9,10,9,4,9,99,-1,8", 9)?, 0);

        assert_eq!(calculate("3,9,7,9,10,9,4,9,99,-1,8", 7)?, 1);
        assert_eq!(calculate("3,9,7,9,10,9,4,9,99,-1,8", 9)?, 0);

        assert_eq!(calculate("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 7)?, 1);
        assert_eq!(calculate("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0)?, 0);
        Ok(())
    }
}