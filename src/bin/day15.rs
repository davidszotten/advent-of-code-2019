use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::{bail, Error};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Add;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Open,
    Destination,
}

impl TryFrom<i64> for Tile {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        use Tile::*;

        match value {
            0 => Ok(Wall),
            1 => Ok(Open),
            2 => Ok(Destination),
            n => bail!("Invalid direction: {}", n),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
    fn as_input(&self) -> i64 {
        use Direction::*;
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }

    fn as_coor(&self) -> Coor {
        use Direction::*;
        match self {
            North => Coor::new(0, -1),
            South => Coor::new(0, 1),
            West => Coor::new(-1, 0),
            East => Coor::new(1, 0),
        }
    }

    fn all() -> [Direction; 4] {
        use Direction::*;
        [North, South, West, East]
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Coor {
    x: i32,
    y: i32,
}

impl Coor {
    fn new(x: i32, y: i32) -> Self {
        Coor { x, y }
    }
}
impl fmt::Debug for Coor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor::new(self.x + other.x, self.y + other.y)
    }
}

fn step(cpu: &mut Cpu, direction: Direction) -> Result<Tile> {
    cpu.enqueue_input(direction.as_input());
    let state = cpu.run()?;
    match state {
        CpuState::Output(value) => Tile::try_from(value),
        s => unreachable!("Invalid state: {:?}", s),
    }
}

fn part1(input: &str) -> Result<usize> {
    use Tile::*;
    let mut cpu = Cpu::from_str(input);
    let mut queue = vec![];
    let start = Coor::default();
    for direction in &Direction::all() {
        queue.push((start, *direction, 1));
    }
    let mut tried = HashMap::new();
    tried.insert(start, Tile::Open);
    let mut path: Vec<(Direction, Coor)> = vec![];
    loop {
        let (pos, direction, distance) = match queue.pop() {
            Some(entry) => entry,
            None => {
                bail!("nowhere else to try");
            }
        };

        while path.len() >= distance {
            let (direction, _) = path.pop().expect("path should not be empty");
            match step(&mut cpu, direction.opposite())? {
                Open => {}
                t => unreachable!("Unexpected tile when going back: {:?}", t),
            };
        }

        let next = pos + direction.as_coor();

        let response = step(&mut cpu, direction)?;
        tried.insert(next, response);
        match response {
            Open => {
                path.push((direction, next));
                for direction in &Direction::all() {
                    if tried.contains_key(&(next + direction.as_coor())) {
                        continue;
                    }
                    queue.push((next, *direction, distance + 1));
                }
            }
            Wall => {}
            Destination => {
                return Ok(distance);
            }
        }
    }
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
