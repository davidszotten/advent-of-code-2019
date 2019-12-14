use aoc2019::cpu::{set_memory, Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::bail;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    Output(((i64, i64), Tile)),
    Score(i64),
    NeedsInput,
    Halted,
}

fn require_output(cpu: &mut Cpu, label: &str) -> Result<i64> {
    Ok(match cpu.run()? {
        CpuState::Output(value) => value,
        _ => bail!("output missing for {}", label),
    })
}

fn tick(mut cpu: &mut Cpu) -> Result<GameState> {
    let state = match cpu.run()? {
        CpuState::Output(x) => {
            let y = require_output(&mut cpu, "y")?;
            if x == -1 {
                let score = require_output(&mut cpu, "score")?;
                GameState::Score(score)
            } else {
                let tile = match require_output(&mut cpu, "tile")? {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    t => bail!("invalid tile {} ({}, {})", t, x, y),
                };
                GameState::Output(((x, y), tile))
            }
        }
        CpuState::NeedsInput => GameState::NeedsInput,
        CpuState::Halted => GameState::Halted,
    };
    Ok(state)
}

fn part1(input: &str) -> Result<usize> {
    let mut cpu = Cpu::from_str(input);
    let mut tiles = HashMap::new();
    while let GameState::Output((pos, tile)) = tick(&mut cpu)? {
        tiles.insert(pos, tile);
    }

    Ok(tiles.values().filter(|&&t| t == Tile::Block).count())
}

fn _draw(tiles: &HashMap<(i64, i64), Tile>) {
    let &(max_x, max_y) = tiles.keys().max().expect("empty");
    let &(min_x, min_y) = tiles.keys().min().expect("empty");

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match tiles.get(&(x, y)).unwrap_or(&Tile::Empty) {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Block => 'X',
                Tile::Paddle => '_',
                Tile::Ball => '0',
            };
            print!("{}", c);
        }
        print!("\n");
    }
}

fn part2(input: &str) -> Result<i64> {
    let mut cpu = Cpu::from_str(input);
    set_memory(&mut cpu, 0, 2);
    // let mut tiles = HashMap::new();
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;
    loop {
        match tick(&mut cpu)? {
            GameState::Output((pos, tile)) => {
                // tiles.insert(pos, tile);
                if tile == Tile::Ball {
                    ball_x = pos.0;
                } else if tile == Tile::Paddle {
                    paddle_x = pos.0;
                }
            }
            GameState::NeedsInput => {
                let input = match ball_x.cmp(&paddle_x) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                cpu.enqueue_input(input);
            }
            GameState::Score(s) => {
                score = s;
            }
            GameState::Halted => {
                break;
            }
        }
    }
    Ok(score)
}
