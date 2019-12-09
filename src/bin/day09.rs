use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::err_msg;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn calculate(program_str: &str, input_values: &[i64]) -> Result<Vec<i64>> {
    let mut cpu = Cpu::from_str(program_str);
    for input_value in input_values.iter() {
        cpu.enqueue_input(*input_value);
    }
    let mut output = vec![];
    loop {
        match cpu.run()? {
            CpuState::Output(value) => output.push(value),
            CpuState::Halted => break,
            _ => unreachable!(),
        }
    }
    Ok(output)
}

fn part1(input: &str) -> Result<i64> {
    calculate(input, &[1])?
        .get(0)
        .ok_or(err_msg("no output"))
        .map(|i| *i)
}

fn part2(input: &str) -> Result<i64> {
    calculate(input, &[2])?
        .get(0)
        .ok_or(err_msg("no output"))
        .map(|i| *i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let output = calculate(&input, &[])?;

        assert_eq!(
            output
                .iter()
                .map(|&x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        );
        assert_eq!(
            calculate("1102,34915192,34915192,7,4,7,99,0", &[])?
                .iter()
                .map(|&x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            "1219070632396864"
        );
        assert_eq!(
            calculate("104,1125899906842624,99", &[])?
                .iter()
                .map(|&x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            "1125899906842624"
        );
        Ok(())
    }
}
