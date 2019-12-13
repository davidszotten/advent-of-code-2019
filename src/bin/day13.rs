use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::err_msg;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn output_or_halt(cpu: &mut Cpu) -> Result<Option<i64>> {
    cpu.run().map(|s| match s {
        CpuState::Output(value) => Some(value),
        CpuState::NeedsInput => unreachable!("wants input"),
        CpuState::Halted => None,
    })
}

fn triple_or_halt(mut cpu: &mut Cpu) -> Result<Option<(i64, i64, i64)>> {
    if let Some(v1) = output_or_halt(&mut cpu)? {
        let v2 = output_or_halt(&mut cpu)?.ok_or(err_msg("halted before v2"))?;
        let v3 = output_or_halt(&mut cpu)?.ok_or(err_msg("halted before v2"))?;
        return Ok(Some((v1, v2, v3)));
    }
    Ok(None)
}

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn part1(input: &str) -> Result<usize> {
    let mut cpu = Cpu::from_str(input);
    let mut tiles = HashMap::new();
    while let Some((x, y, tile)) = triple_or_halt(&mut cpu)? {
        tiles.insert((x, y), tile);
    }

    Ok(tiles.values().filter(|&&t| t == 2).count())
}

fn draw(tiles: &HashMap<(i64, i64), Tile>) {
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
    let mut tiles = HashMap::new();
    while let Some((x, y, tile)) = triple_or_halt(&mut cpu)? {
        let tile = match tile {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            t => unreachable!("invalid tile {}", t),
        };
        tiles.insert((x, y), tile);
    }
    draw(&tiles);

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
