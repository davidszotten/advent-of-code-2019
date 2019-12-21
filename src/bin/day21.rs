// use aoc2019::coor::Coor;
use aoc2019::cpu::Cpu;
use aoc2019::{dispatch, Result};
// use failure::bail;
// use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut cpu = Cpu::from_str(input);
    cpu.expect_ascii("Input instructions:\n")?;
    cpu.write_ascii(
        "NOT B J
NOT C T
OR T J
OR D J
WALK\n",
    );
    cpu.read_ascii()?;
    dbg!(cpu.time_elapsed());
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("")?, 0);
        Ok(())
    }
}
