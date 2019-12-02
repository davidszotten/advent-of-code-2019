use failure::{bail, Error};
use std::convert::TryFrom;

use aoc2019::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Halt,
}

impl TryFrom<i32> for Op {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        use Op::*;

        match value {
            1 => Ok(Add),
            2 => Ok(Mul),
            99 => Ok(Halt),
            _ => bail!("Invalid op code"),
        }
    }
}

fn run(program: Vec<i32>, noun: i32, verb: i32) -> Result<i32> {
    let mut program = program;
    program[1] = noun;
    program[2] = verb;

    let mut pc = 0;
    loop {
        let op = Op::try_from(program[pc])?;
        use Op::*;
        if op == Halt {
            break;
        }

        let left_idx = program[pc + 1] as usize;
        let right_idx = program[pc + 2] as usize;
        let dest_idx = program[pc + 3] as usize;
        let left = program[left_idx];
        let right = program[right_idx];
        program[dest_idx] = if op == Add {
            left + right
        } else {
            left * right
        };
        pc += 4;
        // dbg!(&pc);
    }
    // dbg!(&program);
    Ok(program[0])
}

fn part1(input: &str) -> Result<i32> {
    let program: Vec<_> = input
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    run(program, 12, 2)
}

fn part2(input: &str) -> Result<i32> {
    let program: Vec<_> = input
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            if let Ok(result) = run(program.clone(), noun, verb) {
                if result == 19690720 {
                    return Ok(100 * noun + verb);
                }
            }
        }
    }
    bail!("uh oh")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("1,9,10,3,2,3,11,0,99,30,40,50")?, 3500);
        assert_eq!(part1("1,0,0,0,99")?, 2);
        assert_eq!(part1("2,3,0,3,99")?, 2);
        assert_eq!(part1("2,4,4,5,99,0")?, 2);
        assert_eq!(part1("1,1,1,4,99,5,6,0,99")?, 30);
        Ok(())
    }
}
