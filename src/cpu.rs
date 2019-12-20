use super::Result;
use failure::{bail, err_msg, Error};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        use Mode::*;

        Ok(match value {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => bail!("Invalid op code"),
        })
    }
}

struct Modes {
    value: i64,
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
    fn new(value: i64) -> Self {
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
    AdjustRelativeBase(Mode),
    Halt,
}

impl TryFrom<i64> for Op {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
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
            9 => AdjustRelativeBase(modes.get()?),
            99 => Halt,
            _ => bail!("Invalid op code"),
        };
        Ok(op)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuState {
    Output(i64),
    NeedsInput,
    Halted,
}

#[derive(Clone)]
pub struct Cpu {
    pc: usize,
    program: Vec<i64>,
    input: VecDeque<i64>,
    relative_base: i64,
    memory: HashMap<i64, i64>,
}

impl Cpu {
    fn new(program: Vec<i64>) -> Self {
        Cpu {
            pc: 0,
            program,
            input: VecDeque::new(),
            relative_base: 0,
            memory: HashMap::new(),
        }
    }

    pub fn from_str(program_str: &str) -> Self {
        let program: Vec<_> = program_str
            .split(',')
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        Self::new(program)
    }

    pub fn enqueue_input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    fn get(&self, mode: Mode, source: i64) -> i64 {
        match mode {
            Mode::Immediate => source,
            Mode::Position => self.get_mem(source),
            Mode::Relative => self.get_mem(self.relative_base + source),
        }
    }

    fn get_mem(&self, source: i64) -> i64 {
        *self
            .program
            .get(source as usize)
            .unwrap_or(self.memory.get(&source).unwrap_or(&0))
    }

    fn set(&mut self, mode: Mode, destination: i64, value: i64) {
        let destination = match mode {
            Mode::Immediate => unreachable!("set called with immediate mode"),
            Mode::Position => destination,
            Mode::Relative => self.relative_base + destination,
        };
        assert!(destination >= 0);
        if destination as usize >= self.program.len() {
            self.memory.insert(destination, value);
        } else {
            self.program[destination as usize] = value;
        }
    }

    pub fn run(&mut self) -> Result<CpuState> {
        let state = loop {
            let op = Op::try_from(self.program[self.pc])?;
            // dbg!(self.pc, &op, &self.program);
            use Op::*;
            match op {
                Add(mode1, mode2, mode3) => {
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(mode3, c, self.get(mode1, a) + self.get(mode2, b));
                    self.pc += 4;
                }
                Mul(mode1, mode2, mode3) => {
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(mode3, c, self.get(mode1, a) * self.get(mode2, b));
                    self.pc += 4;
                }
                Input(mode) => {
                    let a = self.program[self.pc + 1];
                    match self.input.pop_front() {
                        None => break CpuState::NeedsInput,
                        Some(value) => {
                            self.set(mode, a, value);
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
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(
                        mode3,
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
                    let a = self.program[self.pc + 1];
                    let b = self.program[self.pc + 2];
                    let c = self.program[self.pc + 3];
                    self.set(
                        mode3,
                        c,
                        if self.get(mode1, a) == self.get(mode2, b) {
                            1
                        } else {
                            0
                        },
                    );
                    self.pc += 4;
                }
                AdjustRelativeBase(mode) => {
                    let a = self.program[self.pc + 1];
                    self.relative_base += self.get(mode, a);
                    self.pc += 2;
                }

                Halt => break CpuState::Halted,
            }
        };
        Ok(state)
    }
}

pub fn read_memory(cpu: &Cpu, position: usize) -> i64 {
    cpu.get_mem(position as i64)
}

pub fn set_memory(cpu: &mut Cpu, position: usize, value: i64) {
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
        assert_eq!(Op::try_from(203)?, Op::Input(Mode::Relative));
        Ok(())
    }

    #[test]
    fn test_203() -> Result<()> {
        let mut cpu = Cpu::from_str("203,10,99");
        cpu.relative_base = 2;
        cpu.enqueue_input(1);
        assert_eq!(cpu.run()?, CpuState::Halted);
        assert_eq!(read_memory(&cpu, 12), 1);
        Ok(())
    }
}
