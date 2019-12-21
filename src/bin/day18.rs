use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Open,
    Wall,
    // Entrance,
    Key(char),
    Door(char),
}

fn parse(input: &str) -> (HashMap<Coor, Tile>, Coor) {
    let mut map = HashMap::new();
    let mut pos = Coor::new(0, 0);
    let mut entrance = pos;
    for c in input.chars() {
        if c == '\n' {
            pos = Coor::new(0, pos.y + 1);
        } else {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                '@' => {
                    entrance = pos;
                    Tile::Open
                }
                c if 'a' <= c && c <= 'z' => Tile::Key(c),
                c if 'A' <= c && c <= 'Z' => Tile::Door(c.to_ascii_lowercase()),
                c => unreachable!("invalid char: {}", c),
            };
            map.insert(pos, tile);
            pos += Coor::new(1, 0);
        }
    }
    (map, entrance)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    pos: Coor,
    keys: u32,
    distance: usize,
}

impl State {
    fn new(pos: &Coor, keys: u32, distance: usize) -> Self {
        Self {
            pos: pos.clone(),
            keys,
            distance,
        }
    }

    fn seen_key(&self) -> (Coor, u32) {
        (self.pos, self.keys)
    }
}

fn key_bits(key: char) -> u32 {
    1 << (key as u8 - 'a' as u8)
}

// fn available(map: &HashMap<Coor, Tile>, state: &State) -> Vec<Coor> {
//     [
//         Coor::new(-1, 0),
//         Coor::new(1, 0),
//         Coor::new(0, -1),
//         Coor::new(0, 1),
//     ]
//     .iter()
//     .map(|&d| state.pos + d)
//     .filter(|&new| match *map.get(&new).expect("not in map") {
//         Tile::Open => true,
//         Tile::Key(_) => true,
//         Tile::Door(c) => state.keys.contains(&c),
//         Tile::Wall => false,
//     })
// }

fn part1(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let keys = map
        .values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(key_bits(*c)),
            _ => None,
        })
        .sum();

    // dbg!(&keys);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(State::new(&entrance, 0, 0));
    while let Some(mut state) = queue.pop_front() {
        if state.distance > 1000 {
            // break;
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        seen.insert(state.seen_key());
        match map.get(&state.pos).expect("not in map") {
            Tile::Open => {}
            Tile::Wall => unreachable!("wall"),
            Tile::Key(c) => {
                state.keys |= key_bits(*c);
            }
            Tile::Door(_) => {}
        }
        if state.keys == keys {
            return Ok(state.distance);
        }
        // for new in available(&map, &state) {
        //
        for mv in [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ]
        .iter()
        {
            let new_pos = state.pos + *mv;
            if !match *map.get(&new_pos).expect("not in map") {
                Tile::Open => true,
                Tile::Key(_) => true,
                Tile::Door(c) => state.keys & key_bits(c) != 0,
                Tile::Wall => false,
            } {
                continue;
            }

            let new_state = State::new(&new_pos, state.keys, state.distance + 1);
            if !seen.contains(&new_state.seen_key()) {
                queue.push_back(new_state);
            }
        }
    }
    Ok(0)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "#########
#b.A.@.a#
#########"
            )?,
            8
        );
        Ok(())
    }

    #[test]
    fn test_part1b() -> Result<()> {
        assert_eq!(
            part1(
                "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
            )?,
            132
        );
        Ok(())
    }

    #[test]
    fn test_part1c() -> Result<()> {
        assert_eq!(
            part1(
                "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
            )?,
            81
        );
        Ok(())
    }
}