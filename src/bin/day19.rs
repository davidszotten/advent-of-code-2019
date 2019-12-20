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

    fn get(&mut self, x: i64, y: i64) -> bool {
        let coor = Coor::new(x, y);
        if let Some(&value) = self.data.get(&coor) {
            return value;
        }
        let mut cpu = self.cpu.clone();
        cpu.enqueue_input(coor.x);
        cpu.enqueue_input(coor.y);

        let value = match cpu.run().expect("run failed") {
            CpuState::Output(value) => match value {
                0 => false,
                1 => true,
                _ => unreachable!("invalid output value"),
            },
            s => panic!("state: {:?}", s),
        };
        self.data.insert(coor, value);
        value
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut map = Map::new(input);
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

fn _display(map: &Map, distance: i64) {
    for y in 0..distance {
        for x in 0..distance {
            print!(
                "{}",
                match map.data.get(&Coor::new(x, y)) {
                    None => " ",
                    Some(true) => "#",
                    Some(false) => ".",
                }
            )
        }
        print!("\n");
    }
    println!("\n");
}

fn find_square(input: &str, size: i64) -> Result<i64> {
    let mut map = Map::new(input);

    let mut prev_x = 0;
    'y: for start_y in 0.. {
        // display(&map, start_y + 1);

        let mut start_x = prev_x;
        while !map.get(start_x, start_y) {
            start_x += 1;
            if start_x > start_y * 4 {
                continue 'y;
            }
        }
        prev_x = start_x;
        while map.get(start_x + size - 1, start_y) {
            if !(map.get(start_x, start_y)
                && map.get(start_x, start_y + size - 1)
                && map.get(start_x + size - 1, start_y + size - 1))
            {
                start_x += 1;
                continue;
            }
            let mut square = true;
            'square: for x in start_x..(start_x + size) {
                for y in start_y..(start_y + size) {
                    if !map.get(x, y) {
                        square = false;
                        break 'square;
                    }
                }
            }
            if square {
                // display(&map, start_y + (1 + size) * 2);
                return Ok(start_x * 10_000 + start_y);
            }
            start_x += 1;
        }

        // dbg!(start_y);
    }
    Ok(0)
}

fn part2(input: &str) -> Result<i64> {
    find_square(input, 100)
}
