use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i64> {
    let cpu = Cpu::from_str(input);
    let mut cpus: Vec<_> = (0..50).map(|_| cpu.clone()).collect();
    let mut buffers: Vec<VecDeque<i64>> = (0..50).map(|_| VecDeque::new()).collect();
    for i in 0..50 {
        cpus[i].enqueue_input(i as i64);
    }
    'main: loop {
        // for (index, cpu) in cpus.iter().enumerate() {
        for index in 0..50 {
            let cpu = &mut cpus[index];
            match cpu.run()? {
                CpuState::NeedsInput => {
                    cpu.enqueue_input(-1);
                }
                CpuState::Output(value) => {
                    buffers[index].push_back(value);
                    if buffers[index].len() == 3 {
                        let destination = buffers[index].pop_front().unwrap();
                        let x = buffers[index].pop_front().unwrap();
                        let y = buffers[index].pop_front().unwrap();
                        if destination == 255 {
                            break 'main (Ok(y));
                        }
                        cpus[destination as usize].enqueue_input(x);
                        cpus[destination as usize].enqueue_input(y);
                    }
                }
                CpuState::Halted => {}
            }
        }
    }
}

fn part2(input: &str) -> Result<i64> {
    let cpu = Cpu::from_str(input);
    let mut cpus: Vec<_> = (0..50).map(|_| cpu.clone()).collect();
    let mut buffers: Vec<VecDeque<i64>> = (0..50).map(|_| VecDeque::new()).collect();
    let mut inputs_since_last_output = [0; 50];
    for i in 0..50 {
        cpus[i].enqueue_input(i as i64);
    }
    let mut nat = None;
    let mut seen_ys = HashSet::new();

    // let mut round = 0;
    'main: loop {
        // round += 1;
        if inputs_since_last_output.iter().all(|&c| c > 2) {
            // dbg!("idle", round);
            let (x, y) = nat.expect("read nat");
            if seen_ys.contains(&y) {
                break Ok(y);
            }
            seen_ys.insert(y);
            cpus[0].enqueue_input(x);
            cpus[0].enqueue_input(y);
        }
        for index in 0..50 {
            let cpu = &mut cpus[index];
            match cpu.run()? {
                CpuState::NeedsInput => {
                    inputs_since_last_output[index] += 1;
                    cpu.enqueue_input(-1);
                }
                CpuState::Output(value) => {
                    inputs_since_last_output[index] = 0;
                    buffers[index].push_back(value);
                    if buffers[index].len() == 3 {
                        let destination = buffers[index].pop_front().unwrap();
                        let x = buffers[index].pop_front().unwrap();
                        let y = buffers[index].pop_front().unwrap();
                        if destination == 255 {
                            nat = Some((x, y));
                        } else {
                            cpus[destination as usize].enqueue_input(x);
                            cpus[destination as usize].enqueue_input(y);
                        }
                    }
                }
                CpuState::Halted => {}
            }
        }
    }
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
