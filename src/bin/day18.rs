use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use std::cmp::min;
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
    prev: Coor,
    keys: u32,
    distance: usize,
}

impl State {
    fn new(pos: &Coor, prev: &Coor, keys: u32, distance: usize) -> Self {
        Self {
            pos: pos.clone(),
            prev: prev.clone(),
            keys,
            distance,
        }
    }

    fn seen_key(&self) -> (Coor, u32) {
        (self.pos, self.keys)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State4 {
    pos: [Coor; 4],
    // prev: [Coor; 4],
    robot: usize,
    keys: u32,
    distance: usize,
}

impl State4 {
    fn new(
        pos: [Coor; 4],
        // prev: [Coor; 4],
        robot: usize,
        keys: u32,
        distance: usize,
    ) -> Self {
        Self {
            pos: pos.clone(),
            // prev: prev.clone(),
            robot,
            keys,
            distance,
        }
    }

    fn seen_key(&self) -> (Coor, u32) {
        (self.pos[self.robot], self.keys)
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

fn coor_key(coor: &Coor) -> usize {
    (100 * coor.x + coor.y) as usize
}

#[derive(Debug)]
struct Reachable {
    pos: Coor,
    label: char,
    distance: usize,
}

fn reachable_keys(map: &[Tile], keys: u32, start: Coor) -> Vec<Reachable> {
    let mut reachable = vec![];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, distance)) = queue.pop_front() {
        seen.insert(pos);
        match map[coor_key(&pos)] {
            Tile::Wall => unreachable!("wall"),
            Tile::Key(c) => {
                if keys & key_bits(c) == 0 {
                    reachable.push(Reachable {
                        pos,
                        label: c,
                        distance,
                    });
                    continue;
                }
            }
            Tile::Door(_) | Tile::Open => {}
        }
        let neighbours = [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ];

        for mv in neighbours.iter() {
            let new_pos = pos + *mv;
            if !match map[coor_key(&new_pos)] {
                Tile::Open => true,
                Tile::Key(_) => true,
                Tile::Door(c) => keys & key_bits(c) != 0,
                Tile::Wall => false,
            } {
                continue;
            }

            if seen.contains(&new_pos) {
                continue;
            }
            queue.push_back((new_pos, distance + 1));
        }
    }
    reachable
}

fn part1(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let keys = map
        .values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(key_bits(*c)),
            _ => None,
        })
        .sum();
    let mut map_v = vec![Tile::Wall; 100 * 100];
    for (coor, tile) in &map {
        map_v[coor_key(coor)] = *tile;
    }

    let mut shortest = None;
    // first find reachable keys, then just bfs key choices

    // dbg!(&keys);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    for reachable in reachable_keys(&map_v, 0, entrance) {
        queue.push_back(State::new(
            &reachable.pos,
            &entrance,
            key_bits(reachable.label),
            reachable.distance,
        ));
    }
    while let Some(state) = queue.pop_front() {
        if state.distance > 1000 {
            // break;
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        if state.keys == keys {
            // dbg!(state.history);
            // return Ok(state.distance);
            // dbg!(state.distance);
            shortest = Some(match shortest {
                None => state.distance,
                Some(d) => min(d, state.distance),
            })
        }
        seen.insert(state.seen_key());

        dbg!(&state);
        for reachable in reachable_keys(&map_v, state.keys, state.pos) {
            // dbg!(&reachable);
            let new_state = State::new(
                &reachable.pos,
                &state.pos,
                state.keys | key_bits(reachable.label),
                state.distance + reachable.distance,
            );
            if !seen.contains(&new_state.seen_key()) {
                queue.push_back(new_state);
            }
        }
    }
    Ok(shortest.expect("nothing found"))
}

fn part2(input: &str) -> Result<usize> {
    let (mut map, entrance) = parse(input);

    map.insert(entrance + Coor::new(-1, -1), Tile::Open);
    map.insert(entrance + Coor::new(-1, 0), Tile::Wall);
    map.insert(entrance + Coor::new(-1, 1), Tile::Open);
    map.insert(entrance + Coor::new(0, -1), Tile::Wall);
    map.insert(entrance + Coor::new(0, 1), Tile::Wall);
    map.insert(entrance + Coor::new(0, 0), Tile::Wall);
    map.insert(entrance + Coor::new(1, -1), Tile::Open);
    map.insert(entrance + Coor::new(1, 0), Tile::Wall);
    map.insert(entrance + Coor::new(1, 1), Tile::Open);

    let entrance1 = entrance + Coor::new(-1, -1);
    let entrance2 = entrance + Coor::new(-1, 1);
    let entrance3 = entrance + Coor::new(1, -1);
    let entrance4 = entrance + Coor::new(1, 1);

    let keys = map
        .values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(key_bits(*c)),
            _ => None,
        })
        .sum();
    let coor_key = |coor: &Coor| (100 * coor.x + coor.y) as usize;
    let mut map_v = vec![Tile::Wall; 100 * 100];
    for (coor, tile) in &map {
        map_v[coor_key(coor)] = *tile;
    }

    // dbg!(&keys);
    let mut seen = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];
    let mut queue = VecDeque::new();
    queue.push_back(State4::new(
        [entrance1, entrance2, entrance3, entrance4],
        // [entrance1, entrance2, entrance3, entrance4],
        0,
        0,
        0,
    ));

    while let Some(mut state) = queue.pop_front() {
        // if state.robot == 0 {
        //     dbg!(&state.pos[0], state.keys);
        // }
        if state.distance > 10 {
            // break;
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        // if state.robot == 0 {
        //     dbg!((&state.pos[0], &state.prev[0]));
        // }
        seen[state.robot].insert(state.seen_key());
        // let mut found_key = false;
        // match map.get(&state.pos[state.robot]).expect("not in map") {
        match map_v[coor_key(&state.pos[state.robot])] {
            Tile::Open => {}
            Tile::Wall => unreachable!("wall"),
            Tile::Key(c) => {
                if state.keys & key_bits(c) == 0 {
                    // found_key = true;
                    dbg!(&state);
                }
                state.keys |= key_bits(c);
            }
            Tile::Door(_) => {}
        }
        if state.keys == keys {
            return Ok(state.distance);
        }
        // for new in available(&map, &state) {
        //

        let neighbours = [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ];

        for robot in 0..4 {
            // if neighbours
            //     .iter()
            //     .filter(|n| map_v[coor_key(&(state.pos[robot] + **n))] == Tile::Wall)
            //     .count()
            //     < 2
            // {
            //     seen[state.robot].insert(state.seen_key());
            // }

            for mv in neighbours.iter() {
                let mut new_pos = state.pos.clone();
                new_pos[robot] += *mv;
                if !match *map.get(&new_pos[robot]).expect("not in map") {
                    Tile::Open => true,
                    Tile::Key(_) => true,
                    Tile::Door(c) => state.keys & key_bits(c) != 0,
                    Tile::Wall => false,
                } {
                    continue;
                }
                // if !found_key && new_pos[robot] == state.prev[robot] {
                //     continue;
                // }

                let new_state = State4::new(
                    new_pos,
                    // state.pos,
                    robot,
                    state.keys,
                    state.distance + 1,
                );
                if !seen[new_state.robot].contains(&new_state.seen_key()) {
                    // if found_key || new_pos[robot] != state.prev[robot] {
                    queue.push_back(new_state);
                }
            }
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() -> Result<()> {
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

    // #[test]
    fn test_part1b1() -> Result<()> {
        assert_eq!(
            part1(
                "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################" //  5    10    15   20
            )?,
            132
        );
        Ok(())
    }

    #[test]
    fn test_part1b2() -> Result<()> {
        assert_eq!(
            part1(
                "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################" //  5    10    15   20
            )?,
            86
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
