use failure::bail;

use aoc2019::cpu::{read_memory, set_memory, Cpu, CpuState};
use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut cpu = Cpu::from_str(input);
    set_memory(&mut cpu, 1, 12);
    set_memory(&mut cpu, 2, 2);
    assert_eq!(cpu.run()?, CpuState::Halted);
    Ok(read_memory(&cpu, 0))
}

fn part2(input: &str) -> Result<i32> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut cpu = Cpu::from_str(input);
            set_memory(&mut cpu, 1, noun);
            set_memory(&mut cpu, 2, verb);
            assert_eq!(cpu.run()?, CpuState::Halted);
            if read_memory(&cpu, 0) == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    bail!("Didn't find a match");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> Result<i32> {
        let mut cpu = Cpu::from_str(input);
        assert_eq!(cpu.run()?, CpuState::Halted);
        Ok(read_memory(&cpu, 0))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(run("1,9,10,3,2,3,11,0,99,30,40,50")?, 3500);
        assert_eq!(run("1,0,0,0,99")?, 2);
        assert_eq!(run("2,3,0,3,99")?, 2);
        assert_eq!(run("2,4,4,5,99,0")?, 2);
        assert_eq!(run("1,1,1,4,99,5,6,0,99")?, 30);
        Ok(())
    }
}
