use aoc2019::{dispatch, Result};
use failure::Error;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::ops::AddAssign;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coor {
    x: i32,
    y: i32,
    z: i32,
}

impl Coor {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coor { x, y, z }
    }

    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl FromStr for Coor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
            // <x=12, y=2, z=-13>
                Regex::new(r"<x= *(-?\d+), y= *(-?\d+), z= *(-?\d+)>")
                    .expect("regex create");
        }

        let caps = RE.captures(s).expect("regex match");
        Ok(Self::new(
            caps[1].parse().expect("regex match 1"),
            caps[2].parse().expect("regex match 2"),
            caps[3].parse().expect("regex match 3"),
        ))
    }
}

impl AddAssign for Coor {
    fn add_assign(&mut self, other: Coor) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Moon {
    position: Coor,
    velocity: Coor,
}

impl Moon {
    fn new(position: Coor, velocity: Coor) -> Self {
        Self { position, velocity }
    }

    fn init(position: Coor) -> Self {
        Self::new(position, Coor::new(0, 0, 0))
    }

    fn energy(&self) -> i32 {
        self.position.energy() * self.velocity.energy()
    }
}

impl FromStr for Moon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
            // pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
                Regex::new(r"pos=<x= *(-?\d+), y= *(-?\d+), z= *(-?\d+)>, vel=<x= *(-?\d+), y= *(-?\d+), z= *(-?\d+)>")
                    .expect("regex create");
        }

        let caps = RE.captures(s).expect("regex match");
        Ok(Self::new(
            Coor::new(
                caps[1].parse().expect("regex match 1"),
                caps[2].parse().expect("regex match 2"),
                caps[3].parse().expect("regex match 3"),
            ),
            Coor::new(
                caps[4].parse().expect("regex match 4"),
                caps[5].parse().expect("regex match 5"),
                caps[6].parse().expect("regex match 6"),
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SingleMoon {
    position: i32,
    velocity: i32,
}

fn parse(input: &str) -> Result<Vec<Coor>> {
    input.split('\n').map(|row| row.parse()).collect()
}

fn _parse_moon(input: &str) -> Result<Vec<Moon>> {
    input.split('\n').map(|row| row.parse()).collect()
}

fn make_moons(coors: &[Coor]) -> Vec<Moon> {
    coors.iter().map(|&pos| Moon::init(pos)).collect()
}

fn split_moons(moons: &[Moon]) -> (Vec<SingleMoon>, Vec<SingleMoon>, Vec<SingleMoon>) {
    let moons_x = moons
        .iter()
        .map(|m| SingleMoon {
            position: m.position.x,
            velocity: m.velocity.x,
        })
        .collect();
    let moons_y = moons
        .iter()
        .map(|m| SingleMoon {
            position: m.position.y,
            velocity: m.velocity.y,
        })
        .collect();
    let moons_z = moons
        .iter()
        .map(|m| SingleMoon {
            position: m.position.z,
            velocity: m.velocity.z,
        })
        .collect();
    (moons_x, moons_y, moons_z)
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i..moons.len() {
            if moons[i].position.x > moons[j].position.x {
                moons[i].velocity.x -= 1;
                moons[j].velocity.x += 1;
            } else if moons[i].position.x < moons[j].position.x {
                moons[i].velocity.x += 1;
                moons[j].velocity.x -= 1;
            }

            if moons[i].position.y > moons[j].position.y {
                moons[i].velocity.y -= 1;
                moons[j].velocity.y += 1;
            } else if moons[i].position.y < moons[j].position.y {
                moons[i].velocity.y += 1;
                moons[j].velocity.y -= 1;
            }

            if moons[i].position.z > moons[j].position.z {
                moons[i].velocity.z -= 1;
                moons[j].velocity.z += 1;
            } else if moons[i].position.z < moons[j].position.z {
                moons[i].velocity.z += 1;
                moons[j].velocity.z -= 1;
            }
        }
    }
}

fn apply_gravity_single(moons: &mut Vec<SingleMoon>) {
    for i in 0..moons.len() {
        for j in i..moons.len() {
            if moons[i].position > moons[j].position {
                moons[i].velocity -= 1;
                moons[j].velocity += 1;
            } else if moons[i].position < moons[j].position {
                moons[i].velocity += 1;
                moons[j].velocity -= 1;
            }
        }
    }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
    for moon in moons.iter_mut() {
        moon.position += moon.velocity;
    }
}

fn apply_velocity_single(moons: &mut Vec<SingleMoon>) {
    for moon in moons.iter_mut() {
        moon.position += moon.velocity;
    }
}

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _ in 0..steps {
        apply_gravity(moons);
        apply_velocity(moons);
    }
}

fn simulate_single(moons: &mut Vec<SingleMoon>) -> usize {
    let initial = moons.clone();

    let mut steps = 0;
    loop {
        apply_gravity_single(moons);
        apply_velocity_single(moons);
        steps += 1;
        if moons
            .iter()
            .zip(initial.iter())
            .all(|(a, b)| a.position == b.position && a.velocity == b.velocity)
        {
            break;
        }
    }
    steps
}

fn part1(input: &str) -> Result<i32> {
    let mut moons = make_moons(&parse(input)?);
    simulate(&mut moons, 1000);
    Ok(moons.iter().map(|m| m.energy()).sum::<i32>())
}

fn part2(input: &str) -> Result<usize> {
    let moons = make_moons(&parse(input)?);
    let (mut moons_x, mut moons_y, mut moons_z) = split_moons(&moons);
    let steps_x = simulate_single(&mut moons_x);
    let steps_y = simulate_single(&mut moons_y);
    let steps_z = simulate_single(&mut moons_z);
    Ok(lcm(lcm(steps_x, steps_y), steps_z))
}

#[cfg(test)]
mod day_12_tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            parse(
                "<x=-1, y=7, z=3>
<x=12, y=2, z=-13>
<x=14, y=18, z=-8>
<x=17, y=4, z=-4>"
            )?,
            vec![
                Coor::new(-1, 7, 3),
                Coor::new(12, 2, -13),
                Coor::new(14, 18, -8),
                Coor::new(17, 4, -4),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_from_str() -> Result<()> {
        let coor: Coor = "<x=-1, y=7, z=3>".parse()?;
        assert_eq!(coor, Coor::new(-1, 7, 3));
        Ok(())
    }

    #[test]
    fn test_simulate() -> Result<()> {
        let coors = parse(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )?;

        let expected1 = "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>";

        let expected5 = "pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>";

        let moons1: Vec<Moon> = _parse_moon(expected1)?;
        let mut moons = make_moons(&coors);
        simulate(&mut moons, 1);
        assert_eq!(moons, moons1);

        let moons5: Vec<Moon> = _parse_moon(expected5)?;
        let mut moons = make_moons(&coors);
        simulate(&mut moons, 5);
        assert_eq!(moons, moons5);

        let mut moons = make_moons(&coors);
        simulate(&mut moons, 10);
        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i32>(), 179);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
            )?,
            2772
        );
        Ok(())
    }
}
