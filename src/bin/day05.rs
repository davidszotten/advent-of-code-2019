use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn calculate(program_str: &str, input_value: i64) -> Result<i64> {
    let mut cpu = Cpu::from_str(program_str);
    cpu.enqueue_input(input_value);
    cpu.run().map(|res| match res {
        CpuState::Output(output) => output,
        _ => unreachable!(),
    })
}

fn part1(input: &str) -> Result<i64> {
    let mut cpu = Cpu::from_str(input);
    cpu.enqueue_input(1);
    let mut outputs = vec![];
    loop {
        match cpu.run()? {
            CpuState::Output(output) => outputs.push(output),
            CpuState::Halted => break,
            _ => unreachable!(),
        }
    }
    let last = outputs.pop().expect("No outputs found");
    assert_eq!(outputs.iter().all(|&x| x == 0), true);
    Ok(last)
}

fn part2(input: &str) -> Result<i64> {
    calculate(input, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

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
