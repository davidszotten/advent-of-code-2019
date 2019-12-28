use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use failure::err_msg;
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
    fn _new(pos: &Coor, prev: &Coor, keys: u32, distance: usize) -> Self {
        Self {
            pos: pos.clone(),
            prev: prev.clone(),
            keys,
            distance,
        }
    }

    fn _seen_key(&self) -> (Coor, u32) {
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
    fn _new(pos: [Coor; 4], _prev: [Coor; 4], robot: usize, keys: u32, distance: usize) -> Self {
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

fn _key_char(key: u32) -> char {
    let mut n = 1;
    let mut key = key;
    while key > 1 {
        n += 1;
        key >>= 1;
    }
    (n - 1 + 'a' as u8) as char
}

#[derive(PartialEq, Eq, Clone)]
struct StateL {
    order: Vec<char>,
    keys: HashSet<char>,
    distance: usize,
}

impl StateL {
    fn _new(prev: &StateL, next: char, distance: usize) -> Self {
        let mut order = prev.order.clone();
        order.push(next);
        let mut keys = prev.keys.clone();
        assert!(!keys.contains(&next));
        keys.insert(next);

        StateL {
            order,
            keys,
            distance: prev.distance + distance,
        }
    }

    fn _first() -> Self {
        StateL {
            order: vec![],
            keys: HashSet::new(),
            distance: 0,
        }
    }
}

impl std::hash::Hash for StateL {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.keys
            .iter()
            .fold(0, |a, &x| a | key_bits(x))
            .hash(state);
    }
}

impl Ord for StateL {
    fn cmp(&self, other: &StateL) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for StateL {
    fn partial_cmp(&self, other: &StateL) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
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

fn _requirements(
    map: &[Tile],
    start: &Coor,
    end: &Coor,
) -> Option<(HashSet<char>, HashSet<char>, usize)> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, HashSet::new(), HashSet::new(), 0));
    while let Some((pos, required_keys, found_keys, distance)) = queue.pop_front() {
        seen.insert(pos);
        let neighbours = [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ];

        for mv in neighbours.iter() {
            let new_pos = pos + *mv;
            let mut required_key = None;
            let mut found_key = None;
            match map[coor_key(&new_pos)] {
                Tile::Open => {}
                Tile::Key(c) => found_key = Some(c),
                Tile::Door(c) => required_key = Some(c),
                Tile::Wall => continue,
            }
            if new_pos == *end {
                return Some((required_keys, found_keys, distance + 1));
            }

            if seen.contains(&new_pos) {
                continue;
            }
            let mut required_keys2 = required_keys.clone();
            let mut found_keys2 = found_keys.clone();
            if let Some(required_key) = required_key {
                required_keys2.insert(required_key);
            }
            if let Some(found_key) = found_key {
                found_keys2.insert(found_key);
            }
            queue.push_back((new_pos, required_keys2, found_keys2, distance + 1));
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reachable {
    pos: Coor,
    required_keys: HashSet<char>,
    found_keys: HashSet<char>,
    key: char,
    distance: usize,
}

fn _all_requirements(
    map: &HashMap<Coor, Tile>,
    entrances: &[Coor],
) -> HashMap<Coor, Vec<Reachable>> {
    let keys_coors: Vec<(Coor, char)> = map
        .iter()
        .filter_map(|(c, v)| match v {
            Tile::Key(k) => Some((*c, *k)),
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
    let mut starts = keys_coors.iter().map(|(c, _)| *c).collect::<Vec<_>>();
    for entrance in entrances {
        starts.push(*entrance);
    }
    let map_v = map_vec(&map);
    let mut reqs = HashMap::new();
    for &left in starts.iter() {
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
            if let Some((required_keys, found_keys, distance)) =
                _requirements(&map_v, &left, &right)
            {
                reachable.push(Reachable {
                    pos: right,
                    required_keys,
                    found_keys,
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

fn find_all_keys(map: &HashMap<Coor, Tile>) -> HashSet<char> {
    map.values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(*c),
            _ => None,
        })
        .collect()
}

// 2006 too high
// 2114 ?!
enum _Action {
    Insert,
    Replace,
    Skip,
}

fn find_from(
    map: &HashMap<Coor, Tile>,
    reachable_map: &HashMap<Coor, Vec<Reachable>>,
    all_keys: &HashSet<char>,
    key: char,
    keys: HashSet<char>,
    mut cache: &mut HashMap<(char, Vec<char>), usize>,
) -> Option<usize> {
    // assume we're at `key`, and have everything but `keys`

    // println!("find_from {}: {}", key, &keys.iter().collect::<String>());

    if keys.is_empty() {
        return Some(0);
    }
    let mut key_v: Vec<_> = (&keys).iter().map(|&c| c).collect();
    key_v.sort();
    if let Some(&distance) = cache.get(&(key, key_v.clone())) {
        return Some(distance);
    }

    let key_coor = map
        .iter()
        .filter_map(|(k, &v)| if v == Tile::Key(key) { Some(k) } else { None })
        .next()
        .expect("key not found");

    let reachables = match reachable_map.get(&key_coor) {
        None => {
            return None;
        }
        Some(reachables) => reachables,
    };

    reachables
        .iter()
        .filter(|r| keys.contains(&r.key))
        .filter(|r| {
            let mut have: HashSet<_> = all_keys.difference(&keys).cloned().collect();
            have.insert(key);

            r.required_keys.difference(&have).count() == 0
        })
        .filter_map(|r| {
            let mut new_set: HashSet<char> = keys.difference(&r.found_keys).cloned().collect();
            new_set.remove(&r.key);
            find_from(&map, &reachable_map, all_keys, r.key, new_set, &mut cache)
                .map(|d| d + r.distance)
        })
        // .inspect(|r| {})
        .min()
        .map(|distance| {
            cache.insert((key, key_v), distance);

            distance
        })
}

fn find_from4(
    map: &HashMap<Coor, Tile>,
    reachable_map: &HashMap<Coor, Vec<Reachable>>,
    all_keys: &HashSet<char>,
    key: [char; 4],
    keys: HashSet<char>,
    mut cache: &mut HashMap<(char, Vec<char>), usize>,
) -> Option<usize> {
    // assume we're at `key`, and have everything but `keys`

    println!("find_from {:?}: {}", key, &keys.iter().collect::<String>());

    if keys.is_empty() {
        return Some(0);
    }
    let mut key_v: Vec<_> = (&keys).iter().map(|&c| c).collect();
    key_v.sort();

    let mut tmp: Vec<usize> = vec![];
    // for &key in key.iter() {
    for idx in 0..4 {
        let prev_key = key.clone();
        let key = key[idx];
        if let Some(distance) = cache.get(&(key, key_v.clone())).clone() {
            tmp.push(*distance);
        } else {
            let key_coor = map
                .iter()
                .filter_map(|(k, &v)| if v == Tile::Key(key) { Some(k) } else { None })
                .next()
                .expect("key not found");

            let reachables = match reachable_map.get(&key_coor) {
                None => {
                    return None;
                }
                Some(reachables) => reachables,
            };

            if let Some(distance) = reachables
                .iter()
                .filter(|r| keys.contains(&r.key))
                .filter(|r| {
                    let mut have: HashSet<_> = all_keys.difference(&keys).cloned().collect();
                    have.insert(key);

                    r.required_keys.difference(&have).count() == 0
                })
                .filter_map(|r| {
                    let mut new_set: HashSet<char> =
                        keys.difference(&r.found_keys).cloned().collect();
                    new_set.remove(&r.key);
                    let mut prev_key = prev_key.clone();
                    prev_key[idx] = r.key;
                    find_from4(
                        &map,
                        &reachable_map,
                        all_keys,
                        prev_key,
                        new_set,
                        &mut cache,
                    )
                    .map(|d| d + r.distance)
                })
                // .inspect(|r| {})
                .min()
                .map(|distance| {
                    cache.insert((key, key_v.clone()), distance);

                    distance
                })
            {
                tmp.push(distance);
            }
        }
    }
    tmp.into_iter().min()
}

fn get_reachable_starts<'a>(
    reachable_map: &'a HashMap<Coor, Vec<Reachable>>,
    entrance: &Coor,
) -> Vec<&'a Reachable> {
    let starts = reachable_map
        .get(&entrance)
        .expect("nothing reachable from entrance");
    starts
        .iter()
        .filter(|reachable| reachable.required_keys.len() == 0)
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    // let (map, _entrances) = switch_entrance(map, entrance);
    let reachable_map = _all_requirements(&map, &[entrance]);

    let reachable_starts = get_reachable_starts(&reachable_map, &entrance);
    let all_keys = find_all_keys(&map);
    let mut cache = HashMap::new();
    reachable_starts
        .iter()
        .filter_map(|reachable| {
            let key = reachable.key;
            let mut without = all_keys.clone();
            without.remove(&key);
            find_from(&map, &reachable_map, &all_keys, key, without, &mut cache)
                .map(|d| reachable.distance + d)
        })
        .min()
        .ok_or(err_msg("empty (b)?"))
}

fn part2(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let (map, entrances) = switch_entrance(map, entrance);
    let reachable_map = _all_requirements(&map, &entrances);

    let reachable_starts1 = get_reachable_starts(&reachable_map, &entrances[0]);
    let reachable_starts2 = get_reachable_starts(&reachable_map, &entrances[1]);
    let reachable_starts3 = get_reachable_starts(&reachable_map, &entrances[2]);
    let reachable_starts4 = get_reachable_starts(&reachable_map, &entrances[3]);
    let mut reachable_starts = vec![];
    for s1 in &reachable_starts1 {
        for s2 in &reachable_starts2 {
            for s3 in &reachable_starts3 {
                for s4 in &reachable_starts4 {
                    reachable_starts.push([s1, s2, s3, s4]);
                }
            }
        }
    }
    // dbg!(&reachable_starts1);
    // dbg!(&reachable_starts2);

    // let starts = entrances
    //     .iter()
    //     .map(|entrance| {
    //         reachable_map
    //             .get(&entrance)
    //             .expect("nothing reachable from entrance")
    //     })
    //     .fold(vec![], |mut acc, x| {
    //         acc.append(&mut x.clone());
    //         acc
    //     });
    // dbg!(&starts);
    let all_keys = find_all_keys(&map);
    let mut cache = HashMap::new();
    reachable_starts
        .iter()
        .filter_map(|reachable4| {
            // let keys = &reachable4.iter().map(|r| r.key).collect();
            let keys = [
                reachable4[0].key,
                reachable4[1].key,
                reachable4[2].key,
                reachable4[3].key,
            ];
            let distance: usize = reachable4.iter().map(|r| r.distance).sum();
            let mut without = all_keys.clone();
            without.remove(&keys[0]);
            without.remove(&keys[1]);
            without.remove(&keys[2]);
            without.remove(&keys[3]);
            find_from4(&map, &reachable_map, &all_keys, keys, without, &mut cache)
                .map(|d| distance + d)
        })
        .min()
        .ok_or(err_msg("empty (a)?"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_char() {
        assert_eq!(_key_char(key_bits('a')), 'a');
        assert_eq!(_key_char(key_bits('c')), 'c');
        assert_eq!(_key_char(key_bits('z')), 'z');
    }

    #[test]
    fn test_part1a() -> Result<()> {
        assert_eq!(
            part1(
                "\
#########
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
    fn test_part1d() -> Result<()> {
        assert_eq!(
            part1(
                "\
########################
#c.A.B.b.............@.#
######################.#
#a.....................#
########################"
            )?,
            68
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
