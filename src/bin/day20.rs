use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Open,
    Wall,
    Label(char),
}

fn parse(input: &str) -> HashMap<Coor, Tile> {
    let mut map = HashMap::new();
    let mut pos = Coor::new(0, 0);
    for c in input.chars() {
        if c == '\n' {
            pos = Coor::new(0, pos.y + 1);
        } else {
            if let Some(tile) = match c {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Open),
                ' ' => None,
                c if 'A' <= c && c <= 'Z' => Some(Tile::Label(c)),
                c => unreachable!("invalid char: {}", c),
            } {
                map.insert(pos, tile);
            }
            pos += Coor::new(1, 0);
        }
    }

    let mut labels = HashMap::new();
    let mut to_remove = vec![];
    for (&coor, &tile) in &map {
        if let Tile::Label(c) = tile {
            let up = coor + Coor::new(0, -1);
            let down = coor + Coor::new(0, 1);
            let left = coor + Coor::new(-1, 0);
            let right = coor + Coor::new(1, 0);
            for (a, b, order) in &[
                (left, right, 0),
                (right, left, 1),
                (up, down, 0),
                (down, up, 1),
            ] {
                if let Some(Tile::Label(d)) = map.get(a) {
                    if let Some(Tile::Open) = map.get(b) {
                        let label = if *order == 0 { (c, *d) } else { (*d, c) };
                        labels.insert(b.clone(), label);
                    }
                    to_remove.push(*a);
                    to_remove.push(coor);
                }
            }
        }
    }
    for coor in to_remove.iter() {
        map.remove(coor);
    }
    dbg!(labels);
    map
}

fn part1(input: &str) -> Result<i32> {
    parse(input);
    Ok(0)
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
