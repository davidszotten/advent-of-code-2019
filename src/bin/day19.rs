use aoc2019::coor::Coor;
use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut cpu = Cpu::from_str(input);
    cpu.run()?;
    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut cpu = cpu.clone();
            cpu.enqueue_input(x);
            cpu.enqueue_input(y);

            match cpu.run()? {
                CpuState::Output(value) => match value {
                    0 => {}
                    1 => {
                        sum += 1;
                    }
                    _ => unreachable!("invalid output value"),
                },
                s => panic!("({},{}), state: {:?}", x, y, s),
            }
        }
    }
    Ok(sum)
}

struct Map {
    cpu: Cpu,
    data: HashMap<Coor, bool>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut cpu = Cpu::from_str(input);
        cpu.run().unwrap();
        let data: HashMap<Coor, bool> = HashMap::new();
        Self { cpu, data }
    }

    fn get(&self, coor: Coor) -> bool {
        if let Some(&value) = self.data.get(&coor) {
            return value;
        }
        let mut cpu = self.cpu.clone();
        cpu.enqueue_input(coor.x);
        cpu.enqueue_input(coor.y);

        match cpu.run().expect("run failed") {
            CpuState::Output(value) => match value {
                0 => false,
                1 => true,
                _ => unreachable!("invalid output value"),
            },
            s => panic!("state: {:?}", s),
        }
    }
}

fn part2(input: &str) -> Result<i32> {
    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {}
    }
    Ok(sum)
}
