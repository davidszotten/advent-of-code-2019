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

fn requirements(map: &[Tile], start: &Coor, end: &Coor) -> Option<(u32, usize)> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0, 0));
    while let Some((pos, keys, distance)) = queue.pop_front() {
        seen.insert(pos);
        let neighbours = [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ];

        for mv in neighbours.iter() {
            let new_pos = pos + *mv;
            let mut key = 0;
            match map[coor_key(&new_pos)] {
                Tile::Open => {}
                Tile::Key(_) => {}
                Tile::Door(c) => key = key_bits(c),
                Tile::Wall => continue,
            }
            if new_pos == *end {
                return Some((keys, distance + 1));
            }

            if seen.contains(&new_pos) {
                continue;
            }
            queue.push_back((new_pos, keys | key, distance + 1));
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reachable {
    pos: Coor,
    required_keys: u32,
    key: u32,
    distance: usize,
}

fn all_requirements(map: &HashMap<Coor, Tile>, entrance: Coor) -> HashMap<Coor, Vec<Reachable>> {
    let mut keys_coors: Vec<(Coor, u32)> = map
        .iter()
        .filter_map(|(c, v)| match v {
            Tile::Key(k) => Some((*c, key_bits(*k))),
            _ => None,
        })
        .collect();
    keys_coors.push((entrance, 0));
    let map_v = map_vec(&map);
    let mut reqs = HashMap::new();
    for &(left, _) in keys_coors.iter() {
        let mut reachable = vec![];
        for &(right, key) in keys_coors.iter() {
            if right == left {
                continue;
            }
            if right == entrance {
                continue;
            }
            if let Some((required_keys, distance)) = requirements(&map_v, &left, &right) {
                reachable.push(Reachable {
                    pos: right,
                    required_keys,
                    key,
                    distance,
                });
            }
        }
        reachable.sort_by_key(|r| r.distance);
        reqs.insert(left, reachable);
    }
    reqs
}

fn map_vec(map: &HashMap<Coor, Tile>) -> Vec<Tile> {
    let mut map_v = vec![Tile::Wall; 100 * 100];
    for (coor, tile) in map {
        map_v[coor_key(coor)] = *tile;
    }
    map_v
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
    // first find reachable keys, then just bfs key choices
    let reachable_map = all_requirements(&map, entrance);

    let mut shortest = None;
    /* let mut seen = HashSet::new(); */
    let mut seens = HashMap::new();
    let mut queue = VecDeque::new();
    for reachable in reachable_map.get(&entrance).expect("entrance missing") {
        if reachable.required_keys != 0 {
            continue;
        }
        queue.push_back(State::new(
            &reachable.pos,
            &entrance,
            reachable.key,
            reachable.distance,
        ));
    }
    let mut max_ones = 0;
    while let Some(state) = queue.pop_front() {
        /* dbg!(&state); */
        let ones = state.keys.count_ones();
        if ones > max_ones {
            max_ones = ones;
            dbg!(ones);
        }
        if state.distance > 1000 {
            /* break; */
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        if state.keys == keys {
            // dbg!(state.history);
            /* return Ok(state.distance); */
            // dbg!(state.distance);
            shortest = Some(match shortest {
                None => state.distance,
                Some(d) => min(d, state.distance),
            })
        }
        /* seen.insert(state.seen_key()); */
        seens
            .entry(state.pos)
            .or_insert(HashSet::new())
            .insert(state.keys);

        if let Some(dist) = shortest {
            if state.distance > dist {
                continue;
            }
        }

        /* let tile = map_v[coor_key(pos)]; */
        /* let key =  */

        /* dbg!(&state); */
        for reachable in reachable_map.get(&state.pos).expect("pos missing") {
            /* dbg!(&reachable); */
            if reachable.pos == entrance {
                println!("skip; entrance");
                continue;
            }
            // dbg!(&reachable);
            // 0011 -> 1100    0001
            if !state.keys & reachable.required_keys != 0 {
                /* println!("skip; not enough keys");         */
                /* println!("{:b}", state.keys);              */
                /* println!("{:b}", reachable.required_keys); */
                continue;
            }

            /* if !state.keys & reachable.key != 0 { */
            if state.keys | reachable.key == state.keys {
                /* println!("skip; already have it"); */
                /* println!("{:b}", state.keys); */
                /* println!("{:b}", reachable.key); */
                /* println!("{:b}", !state.keys); */
                /* println!("{:b}", !state.keys & reachable.key); */
                /* println!("{:b}", state.keys | !reachable.key); */
                continue;
            }

            let new_state = State::new(
                &reachable.pos,
                &state.pos,
                state.keys | reachable.key,
                state.distance + reachable.distance,
            );
            /* dbg!(seen.len()); */
            /* if !seen.contains(&new_state.seen_key()) { */
            if !seens.contains_key(&new_state.pos)
                || !seens
                    .get(&new_state.pos)
                    .expect("contains but no get")
                    .contains(&new_state.keys)
            {
                /* dbg!(&new_state); */
                /* dbg!(&new_state.keys.count_ones()); */
                queue.push_back(new_state);
            } else {
                /* println!("skip; seen"); */
            }
        }
        queue.sort();
    }
    Ok(shortest.expect("nothing found"))
    /* Ok(0) */
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

    #[test]
    fn test_requirements() -> Result<()> {
        let (map, _entrance) = parse(
            "\
#########
#b.A.@.a#
#########",
        );
        let map_v = map_vec(&map);
        assert_eq!(
            requirements(&map_v, &Coor::new(5, 1), &Coor::new(1, 1)),
            Some((key_bits('a'), 4))
        );
        assert_eq!(
            requirements(&map_v, &Coor::new(5, 1), &Coor::new(7, 1)),
            Some((0, 2))
        );
        Ok(())
    }

    #[test]
    fn test_all_requirements() -> Result<()> {
        let (map, entrance) = parse(
            "\
#########
#b.A.@.a#
#########",
        );
        assert_eq!(
            all_requirements(&map, entrance),
            [
                (
                    Coor::new(5, 1),
                    vec![
                        Reachable {
                            pos: Coor::new(7, 1),
                            required_keys: 0,
                            key: key_bits('a'),
                            distance: 2
                        },
                        Reachable {
                            pos: Coor::new(1, 1),
                            required_keys: key_bits('a'),
                            key: key_bits('b'),
                            distance: 4
                        }
                    ]
                ),
                (
                    Coor::new(1, 1),
                    vec![
                        Reachable {
                            pos: Coor::new(5, 1),
                            required_keys: key_bits('a'),
                            key: 0,
                            distance: 4
                        },
                        Reachable {
                            pos: Coor::new(7, 1),
                            required_keys: key_bits('a'),
                            key: key_bits('a'),
                            distance: 6
                        },
                    ]
                ),
                (
                    Coor::new(7, 1),
                    vec![
                        Reachable {
                            pos: Coor::new(5, 1),
                            required_keys: 0,
                            key: 0,
                            distance: 2
                        },
                        Reachable {
                            pos: Coor::new(1, 1),
                            required_keys: key_bits('a'),
                            key: key_bits('b'),
                            distance: 6
                        }
                    ]
                ),
            ]
            .iter()
            .cloned()
            .collect()
        );
        Ok(())
    }

    #[test]
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
            /* 132 */
            0
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
            /* 81 */
            0
        );
        Ok(())
    }
}
