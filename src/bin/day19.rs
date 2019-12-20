use aoc2019::coor::Coor;
use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
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

    fn get(&self, x: i64, y: i64) -> bool {
        let coor = Coor::new(x, y);
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

fn part1(input: &str) -> Result<i32> {
    let map = Map::new(input);
    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            if map.get(x, y) {
                sum += 1
            }
        }
    }
    Ok(sum)
}

fn find_square(input: &str, size: usize) -> Result<usize> {
    let map = Map::new(input);
    let mut distance = 1;
    'main: loop {
        for start_x in 0..distance {
            for start_y in 0..distance {
                let mut square = true;
                'square: for x in start_x..(start_x + size) {
                    for y in start_y..(start_y + size) {
                        if !map.get(x as i64, y as i64) {
                            square = false;
                            break 'square;
                        }
                    }
                }
                if square {
                    break 'main Ok(start_x * 10_000 + start_y);
                }
            }
        }
        distance += 1;
        dbg!(distance);
    }
}

fn part2(input: &str) -> Result<usize> {
    find_square(input, 100)
}
