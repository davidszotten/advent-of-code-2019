use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use std::cmp::Ordering;
// use std::cmp::{min, Ordering};
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

#[derive(PartialEq, Eq, Clone)]
struct State4 {
    pos: [Coor; 4],
    // prev: [Coor; 4],
    robot: usize,
    keys: u32,
    distance: usize,
}

// impl std::hash::Hash for State4 {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.pos.hash(state);
//         self.keys.hash(state);
//     }
// }

impl State4 {
    fn new(pos: [Coor; 4], _prev: [Coor; 4], robot: usize, keys: u32, distance: usize) -> Self {
        Self {
            pos: pos.clone(),
            // prev: prev.clone(),
            robot,
            keys,
            distance,
        }
    }

    // fn seen_key(&self, key_map: u32) -> (Coor, u32) {
    // (self.pos[self.robot], self.keys & key_map)
    // }
    fn _seen_key(&self) -> ([Coor; 4], u32) {
        (self.pos, self.keys)
    }

    fn display_keys(&self) -> String {
        (('a' as u8)..=('z' as u8))
            .map(|c| (c, (1 << c - 'a' as u8)))
            .filter(|(_, k)| self.keys & k != 0)
            .map(|(c, _)| c as char)
            .collect::<String>()
    }
}

impl std::hash::Hash for State4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.keys.hash(state);
    }
}

impl Ord for State4 {
    fn cmp(&self, other: &State4) -> Ordering {
        // other.distance.cmp(&self.distance)
        (other.distance, other.keys).cmp(&(self.distance, self.keys))
    }
}

impl PartialOrd for State4 {
    fn partial_cmp(&self, other: &State4) -> Option<Ordering> {
        Some((other.distance, other.keys).cmp(&(self.distance, self.keys)))
        // Some(other.distance.cmp(&self.distance))
    }
}

impl std::fmt::Debug for State4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "State {{")?;
        writeln!(f, "  robot: {}", self.robot)?;
        writeln!(f, "  {:?}", self.pos)?;
        // writeln!(f, "  {:?} <- {:?}", self.pos, self.prev[self.robot])?;

        write!(f, "  {}\n  {}\n", self.display_keys(), self.distance)?;
        write!(f, "}}\n")
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

fn all_requirements(
    map: &HashMap<Coor, Tile>,
    entrances: &[Coor; 4],
) -> HashMap<Coor, Vec<Reachable>> {
    let mut keys_coors: Vec<(Coor, u32)> = map
        .iter()
        .filter_map(|(c, v)| match v {
            Tile::Key(k) => Some((*c, key_bits(*k))),
            _ => None,
        })
        .collect();
    // let walkable_coors: Vec<Coor> = map
    //     .iter()
    //     .filter_map(|(c, v)| match v {
    //         Tile::Wall => None,
    //         _ => Some(*c),
    //     })
    //     .collect();
    for entrance in entrances {
        keys_coors.push((*entrance, 0));
    }
    let map_v = map_vec(&map);
    let mut reqs = HashMap::new();
    for &(left, _) in keys_coors.iter() {
        // for &left in walkable_coors.iter() {
        let mut reachable = vec![];
        'right: for &(right, key) in keys_coors.iter() {
            if right == left {
                continue;
            }
            for entrance in entrances {
                if right == *entrance {
                    continue 'right;
                }
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
    // let coor_key = |coor: &Coor| (100 * coor.x + coor.y) as usize;
    // let mut map_v = vec![Tile::Wall; 100 * 100];
    // for (coor, tile) in &map {
    //     map_v[coor_key(coor)] = *tile;
    // }
    let map_v = map_vec(&map);

    // dbg!(&keys);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(State::new(&entrance, &entrance, 0, 0));
    while let Some(mut state) = queue.pop_front() {
        if state.distance > 1000 {
            // break;
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        let mut found_key = false;
        // match map.get(&state.pos).expect("not in map") {
        match map_v[coor_key(&state.pos)] {
            Tile::Open => {}
            Tile::Wall => unreachable!("wall"),
            Tile::Key(c) => {
                if state.keys & key_bits(c) == 0 {
                    found_key = true;
                    state.keys |= key_bits(c);
                }
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

        if neighbours
            .iter()
            .filter(|n| map_v[coor_key(&(state.pos + **n))] == Tile::Wall)
            .count()
            < 2
        {
            seen.insert(state.seen_key());
        }

        for mv in neighbours.iter() {
            let new_pos = state.pos + *mv;
            if !match map_v[coor_key(&new_pos)] {
                Tile::Open => true,
                Tile::Key(_) => true,
                Tile::Door(c) => state.keys & key_bits(c) != 0,
                Tile::Wall => false,
            } {
                continue;
            }

            let new_state = State::new(&new_pos, &state.pos, state.keys, state.distance + 1);
            if !found_key && new_pos == state.prev {
                continue;
            }
            if !seen.contains(&new_state.seen_key()) {
                queue.push_back(new_state);
            }
        }
    }
    Ok(0)
}

fn switch_entrance(map: HashMap<Coor, Tile>, entrance: Coor) -> (HashMap<Coor, Tile>, [Coor; 4]) {
    let mut map = map;

    map.insert(entrance + Coor::new(-1, -1), Tile::Open);
    map.insert(entrance + Coor::new(-1, 0), Tile::Wall);
    map.insert(entrance + Coor::new(-1, 1), Tile::Open);
    map.insert(entrance + Coor::new(0, -1), Tile::Wall);
    map.insert(entrance + Coor::new(0, 1), Tile::Wall);
    map.insert(entrance + Coor::new(0, 0), Tile::Wall);
    map.insert(entrance + Coor::new(1, -1), Tile::Open);
    map.insert(entrance + Coor::new(1, 0), Tile::Wall);
    map.insert(entrance + Coor::new(1, 1), Tile::Open);

    let entrances = [
        entrance + Coor::new(-1, -1),
        entrance + Coor::new(-1, 1),
        entrance + Coor::new(1, -1),
        entrance + Coor::new(1, 1),
    ];

    (map, entrances)
}

fn find_all_keys(map: &HashMap<Coor, Tile>) -> u32 {
    map.values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(key_bits(*c)),
            _ => None,
        })
        .sum()
}

// 2006 too high
// 2114 ?!
enum Action {
    Insert,
    Replace,
    Skip,
}

fn part2(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let (map, entrances) = switch_entrance(map, entrance);

    let keys = find_all_keys(&map);
    let reachable_map = all_requirements(&map, &entrances);
    // let map_v = map_vec(&map);

    let mut seen = HashSet::new();
    // dbg!(&reachable_map);
    let mut queue = vec![];
    queue.push(State4::new(entrances, entrances, 0, 0, 0));
    let mut steps = 0;
    while let Some(state) = queue.pop() {
        steps += 1;
        if steps % 1 == 0 {
            dbg!(&state);
            dbg!(&queue.len());
        }
        seen.insert(state.clone());
        // seen.insert(state.keys);
        for robot in 0..4 {
            if let Some(reachables) = reachable_map.get(&state.pos[robot]) {
                for reachable in reachables.iter() {
                    if state.keys & reachable.key != 0 {
                        continue;
                    }
                    if !state.keys & reachable.required_keys != 0 {
                        continue;
                    }
                    let mut new_pos = state.pos.clone();
                    new_pos[robot] = reachable.pos;
                    let new_state = State4::new(
                        new_pos,
                        state.pos,
                        robot,
                        state.keys | reachable.key,
                        state.distance + reachable.distance,
                    );
                    if new_state.keys == keys {
                        dbg!(&new_state);
                        return Ok(new_state.distance);
                    }
                    if seen.contains(&new_state) {
                        // return Ok(1);
                        continue;
                    }
                    seen.insert(new_state.clone());
                    let idx = queue.binary_search(&new_state).unwrap_or_else(|x| x);
                    let mut action = Action::Insert;
                    // dbg!("insert", idx);
                    // for offset in 0..1 {
                    // for offset in 0..min(2, idx) {
                    // if let Some(found) = queue.get(idx - offset) {
                    if let Some(found) = queue.get(idx) {
                        if found.pos == new_state.pos {
                            // println!("{} {}", found.display_keys(), new_state.display_keys());
                            if found.keys == new_state.keys {
                                // dbg!(&found);
                                // dbg!(&new_state);
                                assert!(new_state.distance >= found.distance);
                                action = Action::Skip;
                            // dbg!("skip", idx);
                            } else {
                                if ((!new_state.keys) & found.keys) == 0 {
                                    // dbg!(&found);
                                    // dbg!(&new_state);
                                    assert!(new_state.distance <= found.distance);
                                    action = Action::Replace;
                                // dbg!("replace", idx);
                                } else if (new_state.keys & (!found.keys)) == 0 {
                                    assert!(new_state.distance >= found.distance);
                                    action = Action::Skip;
                                    // dbg!("skip", idx);
                                }
                            }
                        }
                    }
                    // }
                    match action {
                        Action::Replace => queue[idx] = new_state,
                        Action::Insert => {
                            /*
                            if new_state.pos
                                == [
                                    Coor::new(1, 17),
                                    Coor::new(11, 43),
                                    Coor::new(53, 11),
                                    Coor::new(49, 41),
                                ]
                                && new_state.distance == 1466
                            // && new_state.keys == 8392845
                            {
                                dbg!(idx);
                                let found = &queue[idx];
                                dbg!(&new_state);
                                // dbg!(&queue.get(idx - 1));
                                dbg!(&found);
                                // dbg!(&queue.get(idx + 1));

                                dbg!(new_state.display_keys());
                                dbg!(found.display_keys());
                                dbg!(!new_state.keys & found.keys);
                                dbg!(new_state.keys & !found.keys);
                                // return Ok(1);
                            }
                            */
                            queue.insert(idx, new_state);
                        }
                        Action::Skip => {}
                    }
                    // dbg!(&queue);
                }
            }
        }
    }
    Ok(0)
}

fn _part2x(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let (map, entrances) = switch_entrance(map, entrance);

    let keys = find_all_keys(&map);
    let mut key_quadrants = [0; 4];

    for (coor, tile) in &map {
        if let Tile::Door(c) = tile {
            if coor.x < entrance.x && coor.y < entrance.y {
                key_quadrants[0] |= key_bits(*c);
            }
            if coor.x < entrance.x && coor.y > entrance.y {
                key_quadrants[1] |= key_bits(*c);
            }
            if coor.x > entrance.x && coor.y < entrance.y {
                key_quadrants[2] |= key_bits(*c);
            }
            if coor.x > entrance.x && coor.y > entrance.y {
                key_quadrants[3] |= key_bits(*c);
            }
        }
    }
    // println!("{:b}", key_quadrants[0]);
    // println!("{:b}", key_quadrants[1]);
    // println!("{:b}", key_quadrants[2]);
    // println!("{:b}", key_quadrants[3]);

    // let mut map_v = vec![Tile::Wall; 100 * 100];
    // let mut seen_by_coor = vec![None; 100 * 100];
    // for (coor, tile) in &map {
    // map_v[coor_key(coor)] = *tile;
    // }
    let map_v = map_vec(&map);
    let reachable_map = all_requirements(&map, &entrances);

    // dbg!(&keys);
    let mut seen = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];
    let mut queue = VecDeque::new();
    queue.push_back(State4::new(entrances, entrances, 0, 0, 0));
    /* let mut steps = 0; */
    /* let mut max_dist = 0; */
    while let Some(mut state) = queue.pop_front() {
        /* if state.distance > max_dist { */
        // if steps % 1_000_000 == 0 {
        //     /* max_dist = state.distance; */
        //     /* dbg!(state.distance); */
        //     dbg!(steps / 1_000_000, &state);
        // }
        /* steps += 1; */
        if state.distance > 500 {
            /* break; */
            // println!("{}", &state.distance);
            // println!("{:#b}", &state.seen_key().1);
            // println!("{}", &state.seen_key().1.count_ones());
        }
        // if state.robot == 0 {
        //     dbg!((&state.pos[0], &state.prev[0]));
        // }
        dbg!(state.robot, state.pos[state.robot], state.keys,);
        // seen[state.robot].insert(state.seen_key(key_quadrants[state.robot]));
        // seen[state.robot].insert(state.seen_key());
        let mut _found_key = false;
        // match map.get(&state.pos[state.robot]).expect("not in map") {
        match map_v[coor_key(&state.pos[state.robot])] {
            Tile::Open => {}
            Tile::Wall => unreachable!("wall"),
            Tile::Key(c) => {
                if state.keys & key_bits(c) == 0 {
                    _found_key = true;
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

        dbg!(&state);
        // if state.robot == 2 {
        // dbg!(&state);
        // }

        let neighbours = [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ];

        // for robot in 0..4 {
        //     dbg!(reachable_map.get(&state.pos[robot]));
        // }
        // return Ok(0);
        for robot in 0..4 {
            let n_reachable = reachable_map
                .get(&state.pos[robot])
                .unwrap_or(&vec![])
                .iter()
                .filter(|r| r.required_keys & !state.keys == 0)
                .count();
            dbg!(robot, n_reachable);
            if n_reachable == 0 {
                continue;
            }
            // let n_neighbours = neighbours
            //     .iter()
            //     .filter(|n| map_v[coor_key(&(state.pos[robot] + **n))] == Tile::Wall)
            //     .count();
            // if n_neighbours < 2 {
            if true {
                seen[state.robot].insert(state._seen_key());
            // seen[state.robot].insert(state.seen_key(key_quadrants[state.robot]));
            /* dbg!(&seen_by_coor[coor_key(&state.pos[robot])]); */
            // if seen_by_coor[coor_key(&state.pos[robot])] == None {
            //     seen_by_coor[coor_key(&state.pos[robot])] = Some(HashSet::new());
            // }
            // seen_by_coor[coor_key(&state.pos[robot])]
            //     .as_mut()
            //     .expect("no set?")
            //     .insert(state.keys);
            /* dbg!(&seen_by_coor[coor_key(&state.pos[robot])]); */
            } else {
                /* dbg!(                                      */
                /*     "not enought neighbours at {:?} ({})", */
                /*     state.pos[robot],                      */
                /*     n_neighbours                           */
                /* );                                         */
            }

            for mv in neighbours.iter() {
                let mut new_pos = state.pos.clone();
                new_pos[robot] += *mv;
                if !match map_v[coor_key(&new_pos[robot])] {
                    Tile::Open => true,
                    Tile::Key(_) => true,
                    Tile::Door(c) => state.keys & key_bits(c) != 0,
                    Tile::Wall => false,
                } {
                    continue;
                }
                // if !found_key && new_pos[robot] == state.prev[robot] {
                // continue;
                // }

                let new_state =
                    State4::new(new_pos, state.pos, robot, state.keys, state.distance + 1);
                if !seen[new_state.robot]
                    // .contains(&new_state.seen_key(key_quadrants[new_state.robot]))
                    .contains(&new_state._seen_key())
                {
                    // dbg!(
                    //     new_state.keys,
                    //     new_state.robot,
                    //     new_state.pos[new_state.robot]
                    // );
                    // dbg!(&seen_by_coor[coor_key(&new_state.pos[new_state.robot])]);
                    /* if seen_by_coor[coor_key(&new_state.pos[new_state.robot])] == None */
                    //     || !seen_by_coor[coor_key(&new_state.pos[new_state.robot])]
                    //         .as_ref()
                    //         .expect("wasn't none?")
                    //         .contains(&new_state.keys)
                    // {
                    // if found_key || new_pos[robot] != state.prev[robot] {
                    /* dbg!("yes"); */
                    queue.push_back(new_state);
                } else {
                    /* dbg!("no"); */
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
                "\
########################
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
                "\
########################
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

    #[test]
    fn test_part2a() -> Result<()> {
        assert_eq!(
            part2(
                "\
############
#bA..#.#####
#####@######
####.#...a.#
############"
            )?,
            6
        );
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"
            )?,
            8
        );
        Ok(())
    }

    #[test]
    fn test_part2c() -> Result<()> {
        assert_eq!(
            part2(
                "\
#############
#DcBa.#.GhKl#
#.###.#.#I###
#e#d##@##j#k#
###C#.#.###J#
#fEbA.#.FgHi#
#############"
            )?,
            32
        );
        Ok(())
    }

    #[test]
    fn test_part2d() -> Result<()> {
        assert_eq!(
            part2(
                "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba.#.BcIJ#
######@######
#nK.L.#.G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"
            )?,
            // 72
            1
        );
        Ok(())
    }
}
