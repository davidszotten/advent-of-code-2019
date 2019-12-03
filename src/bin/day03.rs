#![feature(generators, generator_trait)]

use aoc2019::{dispatch, Result};
use failure::{bail, Error};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::Add;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        use Direction::*;

        match &value[..1] {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => bail!("Invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Coor {
    x: i32,
    y: i32,
}

impl Coor {
    fn new(x: i32, y: i32) -> Self {
        Coor { x, y }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Coor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor::new(self.x + other.x, self.y + other.y)
    }
}

impl Ord for Coor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance().cmp(&other.distance())
    }
}
impl PartialOrd for Coor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Vector {
    direction: Direction,
    length: i32,
}

impl Vector {
    fn as_coor_magnitude_pair(&self) -> (Coor, i32) {
        // fn as_coor(&self) -> Coor {
        use Direction::*;
        let coor = match &self.direction {
            Up => Coor::new(0, 1),
            Down => Coor::new(0, -1),
            Left => Coor::new(-1, 0),
            Right => Coor::new(1, 0),
        };
        (coor, self.length)
    }
}

impl TryFrom<&str> for Vector {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Vector {
            direction: Direction::try_from(value)?,
            length: i32::from_str_radix(&value[1..], 10)?,
        })
    }
}

fn wire_positions(input: &str) -> HashMap<Coor, i32> {
    let mut pos = Coor::default();
    let mut steps = 0;
    let mut positions = HashMap::new();

    for vector in input.split(',').filter_map(|x| Vector::try_from(x).ok()) {
        let (direction, magnitude) = vector.as_coor_magnitude_pair();
        for _ in 0..magnitude {
            steps += 1;
            pos = pos + direction;
            positions.insert(pos, steps);
        }
    }
    positions
}

fn part1(input: &str) -> Result<i32> {
    let mut wires = input.split('\n');
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();

    let steps1 = wire_positions(wire1);
    let steps2 = wire_positions(wire2);
    let positions1: HashSet<_> = steps1.keys().collect();
    let positions2: HashSet<_> = steps2.keys().collect();

    let intersections = positions1.intersection(&positions2);
    // dbg!(&intersections);
    let mut intersections: Vec<_> = intersections.cloned().collect();
    intersections.sort();
    let smallest = intersections[0];
    Ok(smallest.x + smallest.y)
}

fn part2(input: &str) -> Result<i32> {
    let mut wires = input.split('\n');
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();

    let steps1 = wire_positions(wire1);
    let steps2 = wire_positions(wire2);
    let positions1: HashSet<_> = steps1.keys().collect();
    let positions2: HashSet<_> = steps2.keys().collect();

    let intersections = positions1.intersection(&positions2);
    let mut intersection_steps = vec![];
    for intersection in intersections {
        let count1 = steps1.get(intersection).unwrap();
        let count2 = steps2.get(intersection).unwrap();
        intersection_steps.push(count1 + count2);
    }
    // dbg!(&intersections);
    // let mut intersections: Vec<_> = intersections.cloned().collect();
    intersection_steps.sort();
    let smallest = intersection_steps[0];
    Ok(smallest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() -> Result<()> {
        assert_eq!(Direction::try_from("U21")?, Direction::Up);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            )?,
            6
        );
        assert_eq!(
            part1(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )?,
            159
        );
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )?,
            135
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )?,
            610
        );
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )?,
            410
        );
        Ok(())
    }
}
