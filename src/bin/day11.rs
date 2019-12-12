use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
struct Coor {
    x: i32,
    y: i32,
}

impl Coor {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Robot {
    direction: Direction,
    position: Coor,
}

impl Robot {
    fn new() -> Self {
        Self {
            direction: Direction::Up,
            position: Coor::default(),
        }
    }
}

fn turn_left(direction: &Direction) -> Direction {
    use Direction::*;
    match direction {
        Up => Left,
        Left => Down,
        Down => Right,
        Right => Up,
    }
}

fn turn_right(direction: &Direction) -> Direction {
    use Direction::*;
    match direction {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn mv(position: Coor, direction: Direction) -> Coor {
    use Direction::*;

    match direction {
        Up => Coor::new(position.x, position.y - 1),
        Right => Coor::new(position.x + 1, position.y),
        Down => Coor::new(position.x, position.y + 1),
        Left => Coor::new(position.x - 1, position.y),
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut whites = HashSet::new();
    let mut paints = HashSet::new();

    let mut robot = Robot::new();

    let mut cpu = Cpu::from_str(input);

    loop {
        match cpu.run()? {
            CpuState::Output(colour) => {
                match colour {
                    0 => {
                        whites.remove(&robot.position);
                    }
                    1 => {
                        whites.insert(robot.position);
                    }
                    _ => panic!("Invalid paint colour"),
                }
                paints.insert(robot.position);
                if let CpuState::Output(turn) = cpu.run()? {
                    robot.direction = match turn {
                        0 => turn_left(&robot.direction),
                        1 => turn_right(&robot.direction),
                        _ => unreachable!("Invalid turn direction"),
                    };
                    robot.position = mv(robot.position, robot.direction);
                } else {
                    unreachable!("second output missing");
                }
            }
            CpuState::NeedsInput => cpu.enqueue_input(if whites.contains(&robot.position) {
                1
            } else {
                0
            }),
            CpuState::Halted => break,
        }
    }
    Ok(paints.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut whites = HashSet::new();
    let mut paints = HashSet::new();

    let mut robot = Robot::new();

    let mut cpu = Cpu::from_str(input);

    whites.insert(robot.position);

    loop {
        match cpu.run()? {
            CpuState::Output(colour) => {
                match colour {
                    0 => {
                        whites.remove(&robot.position);
                    }
                    1 => {
                        whites.insert(robot.position);
                    }
                    _ => panic!("Invalid paint colour"),
                }
                paints.insert(robot.position);
                if let CpuState::Output(turn) = cpu.run()? {
                    robot.direction = match turn {
                        0 => turn_left(&robot.direction),
                        1 => turn_right(&robot.direction),
                        _ => unreachable!("Invalid turn direction"),
                    };
                    robot.position = mv(robot.position, robot.direction);
                } else {
                    unreachable!("second output missing");
                }
            }
            CpuState::NeedsInput => cpu.enqueue_input(if whites.contains(&robot.position) {
                1
            } else {
                0
            }),
            CpuState::Halted => break,
        }
    }
    // dbg!(whites);
    let mut v: Vec<_> = whites.iter().collect();
    v.sort_by_key(|t| -t.x);
    let max_x = v[0].x;

    v.sort_by_key(|t| -t.y);
    let max_y = v[0].y;

    // dbg!(&whites.contains(&Coor::new(26, 5)));

    for y in 0..=max_y {
        for x in 0..=max_x {
            // dbg!(x, y);
            let output = if whites.contains(&Coor::new(x, y)) {
                "#"
            } else {
                " "
            };
            print!("{}", output);
        }
        print!("\n");
    }
    Ok(paints.len())
}
