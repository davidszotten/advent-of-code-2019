use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use permutohedron::LexicalPermutation;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn calculate(program_str: &str, input_values: &[i64]) -> Result<i64> {
    let mut cpu = Cpu::from_str(program_str);
    for input_value in input_values.iter() {
        cpu.enqueue_input(*input_value);
    }
    match cpu.run()? {
        CpuState::Output(value) => Ok(value),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> Result<i64> {
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

fn part2(input: &str) -> Result<i64> {
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
            cpus[index].enqueue_input(phase);
        }
        cpus[0].enqueue_input(0);

        let mut running = [true; 5];
        let mut signal = 0;
        for index in (0..5).cycle() {
            if !running.iter().any(|&r| r) {
                break;
            }
            let state = cpus[index].run()?;
            // dbg!(index, state);
            match state {
                CpuState::Halted => running[index] = false,
                CpuState::NeedsInput => {}
                CpuState::Output(value) => {
                    if index == 4 {
                        signal = value;
                    }
                    cpus[(index + 1) % 5].enqueue_input(value);
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

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
