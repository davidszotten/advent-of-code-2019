use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use failure::err_msg;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Open,
    Wall,
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

fn coor_key(coor: &Coor) -> usize {
    (100 * coor.x + coor.y) as usize
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
struct KeyBits {
    n: u32,
}

impl KeyBits {
    fn new(n: u32) -> Self {
        KeyBits { n }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn contains(&self, key: char) -> bool {
        // TODO: check
        !self.n & KeyBits::char_to_bit(key) == 0
    }

    fn char_to_bit(key: char) -> u32 {
        1 << (key as u8 - 'a' as u8)
    }

    fn _bit_to_char(bit: u32) -> char {
        let mut n = 1;
        let mut bit = bit;
        while bit > 1 {
            n += 1;
            bit >>= 1;
        }
        (n - 1 + 'a' as u8) as char
    }

    fn display(&self) -> String {
        (('a' as u8)..=('z' as u8))
            .map(|c| (c, (1 << c - 'a' as u8)))
            .filter(|(_, k)| self.n & k != 0)
            .map(|(c, _)| c as char)
            .collect::<String>()
    }
}

impl std::convert::From<Option<char>> for KeyBits {
    fn from(maybe_char: Option<char>) -> Self {
        match maybe_char {
            Some(c) => KeyBits::default() + c,
            None => KeyBits::default(),
        }
    }
}

impl Add for KeyBits {
    type Output = KeyBits;
    fn add(self, other: KeyBits) -> KeyBits {
        KeyBits::new(self.n | other.n)
    }
}

impl Add<char> for KeyBits {
    type Output = KeyBits;
    fn add(self, other: char) -> KeyBits {
        KeyBits::new(self.n | KeyBits::char_to_bit(other))
    }
}

impl Add<Option<char>> for KeyBits {
    type Output = KeyBits;
    fn add(self, other: Option<char>) -> KeyBits {
        match other {
            Some(c) => self + c,
            None => self,
        }
    }
}

impl Sub for KeyBits {
    type Output = KeyBits;
    fn sub(self, other: KeyBits) -> KeyBits {
        KeyBits::new(self.n & !other.n)
    }
}

impl Sub<char> for KeyBits {
    type Output = KeyBits;
    fn sub(self, other: char) -> KeyBits {
        KeyBits::new(self.n & !KeyBits::char_to_bit(other))
    }
}

impl std::iter::FromIterator<char> for KeyBits {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        KeyBits::new(
            iter.into_iter()
                .map(|c| KeyBits::char_to_bit(c))
                .fold(0, |acc, x| acc | x),
        )
    }
}

impl std::fmt::Debug for KeyBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyBits {{{}}}", self.display())
    }
}

fn requirements(map: &[Tile], start: &Coor, end: &Coor) -> Option<(KeyBits, KeyBits, usize)> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, KeyBits::default(), KeyBits::default(), 0));
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
            queue.push_back((
                new_pos,
                required_keys + KeyBits::from(required_key),
                found_keys + KeyBits::from(found_key),
                distance + 1,
            ));
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reachable {
    pos: Coor,
    required_keys: KeyBits,
    found_keys: KeyBits,
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
            if let Some((required_keys, found_keys, distance)) = requirements(&map_v, &left, &right)
            {
                // if left == Coor::new(7, 3) && right == Coor::new(9, 1) {
                // dbg!(&required_keys, &found_keys, distance);
                // }
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
        // if left == Coor::new(7, 3) {
        //     dbg!(&left, &reachable);
        // }
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

fn find_all_keys(map: &HashMap<Coor, Tile>) -> KeyBits {
    map.values()
        .filter_map(|v| match v {
            Tile::Key(c) => Some(*c),
            _ => None,
        })
        .collect()
}

enum _Action {
    Insert,
    Replace,
    Skip,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum From {
    Entrance(Coor),
    Key(char),
}

fn find_from(
    map: &HashMap<Coor, Tile>,
    reachable_map: &HashMap<Coor, Vec<Reachable>>,
    all_keys: KeyBits,
    from: From,
    keys: KeyBits,
    mut cache: &mut HashMap<(From, KeyBits), usize>,
) -> Option<usize> {
    // assume we're at `key`, and have everything but `keys`

    // println!("find_from {}: {}", key, &keys.iter().collect::<String>());

    if keys.is_empty() {
        return Some(0);
    }
    if let Some(&distance) = cache.get(&(from, keys)) {
        return Some(distance);
    }

    let from_coor = match from {
        From::Entrance(coor) => coor,
        From::Key(key) => map
            .iter()
            .filter_map(|(k, &v)| if v == Tile::Key(key) { Some(*k) } else { None })
            .next()
            .expect("key not found"),
    };

    let reachables = match reachable_map.get(&from_coor) {
        None => {
            return None;
        }
        Some(reachables) => reachables,
    };

    reachables
        .iter()
        .filter(|r| keys.contains(r.key))
        .filter(|r| {
            let have = all_keys - keys
                + match from {
                    From::Key(k) => Some(k),
                    From::Entrance(_) => None,
                };

            (r.required_keys - have).is_empty()
        })
        .filter_map(|r| {
            find_from(
                &map,
                &reachable_map,
                all_keys,
                From::Key(r.key),
                keys - r.found_keys - r.key,
                &mut cache,
            )
            .map(|d| d + r.distance)
        })
        // .inspect(|r| {})
        .min()
        .map(|distance| {
            cache.insert((from, keys), distance);

            distance
        })
}

fn _print_from(from: [From; 4]) {
    print!("[");
    for f in &from {
        match f {
            From::Key(k) => print!("{}, ", k),
            From::Entrance(c) => print!("{:?}, ", c),
        }
    }
    print!("]");
}

fn find_from4(
    map: &HashMap<Coor, Tile>,
    reachable_map: &HashMap<Coor, Vec<Reachable>>,
    all_keys: KeyBits,
    from: [From; 4],
    keys: KeyBits,
    // mut cache: &mut HashMap<(From, Vec<char>), usize>,
    mut cache: &mut HashMap<([From; 4], KeyBits), Option<usize>>,
) -> Option<usize> {
    // assume we're at `key`, and have everything but `keys`

    // print!("find_from4 [");
    // for f in &from {
    //     match f {
    //         From::Key(k) => print!("{}, ", k),
    //         From::Entrance(c) => print!("{:?}, ", c),
    //     }
    // }
    // println!("] to {:?}", &keys);

    // if display_keys(&keys) == "cdfgjklmno".to_string() && from[0] == From::Key('b') {
    //     dbg!(display_keys(&keys), &from);
    // }

    if keys.is_empty() {
        return Some(0);
    }
    if let Some(distance) = cache.get(&(from, keys)) {
        return *distance;
    }

    let from_keys: KeyBits = from
        .iter()
        .filter_map(|f| match f {
            From::Key(key) => Some(*key),
            _ => None,
        })
        .collect();

    let mut tmp: Vec<usize> = vec![];
    for idx in 0..4 {
        let indexed_from = from[idx];
        let from_coor = match indexed_from {
            From::Entrance(coor) => coor,
            From::Key(key) => map
                .iter()
                .filter_map(|(k, &v)| if v == Tile::Key(key) { Some(*k) } else { None })
                .next()
                .expect("key not found"),
        };

        let reachables = match reachable_map.get(&from_coor) {
            None => {
                return None;
            }
            Some(reachables) => reachables,
        };

        if let Some(distance) = reachables
            .iter()
            .filter(|r| keys.contains(r.key))
            .filter(|r| {
                let have = all_keys - keys + from_keys;

                (r.required_keys - have).is_empty()
            })
            .filter_map(|r| {
                let mut prev_from = from.clone();
                prev_from[idx] = From::Key(r.key);
                let res = find_from4(
                    &map,
                    &reachable_map,
                    all_keys,
                    prev_from,
                    keys - r.found_keys - from_keys - r.key,
                    &mut cache,
                );
                // if indexed_from == From::Key('k') && key_v == vec!['j', 'l', 'm', 'n', 'o'] {
                //     print_from(prev_from);
                //     println!("");
                //     println!("{:?}", from_coor);
                //     println!("{:?}", r);
                //     println!("{}", display_keys(&new_set));
                //     dbg!(&res);
                // }
                res.map(|d| (r, d + r.distance))
            })
            .inspect(|(_r, _d)| {
                // dbg!(&r, &keys, d);
                {
                    // println!("{:?}, {}, {}", &r, display_keys(&keys), d)
                };
            })
            .map(|(_, d)| d)
            .min()
            .map(|distance| {
                // if let Some(found) = cache.get(&(indexed_from, key_v.clone())) {
                //     if *found != distance
                //         && key_v == vec!['j', 'l', 'm', 'n', 'o']
                //         && indexed_from == From::Key('k')
                //     {
                //         dbg!(
                //             &indexed_from,
                //             &key_v.iter().collect::<String>(),
                //             distance,
                //             found
                //         );
                //     }
                // } else {
                //     // dbg!(&indexed_from, &key_v, distance, distance);
                //     cache.insert((indexed_from, key_v.clone()), distance);
                // }

                distance
            })
        {
            tmp.push(distance);
        }
    }
    let res = tmp.into_iter().min();

    cache.insert((from, keys), res);

    // print!("find_from4 [");
    // for f in &from {
    //     match f {
    //         From::Key(k) => print!("{}, ", k),
    //         From::Entrance(c) => print!("{:?}, ", c),
    //     }
    // }
    // println!("] to {:?}: {:?}", &keys, res);

    res
}

fn part1(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let reachable_map = _all_requirements(&map, &[entrance]);

    let all_keys = find_all_keys(&map);
    let mut cache = HashMap::new();
    find_from(
        &map,
        &reachable_map,
        all_keys,
        From::Entrance(entrance),
        all_keys,
        &mut cache,
    )
    .ok_or(err_msg("empty pt1"))
}

fn part2(input: &str) -> Result<usize> {
    let (map, entrance) = parse(input);
    let (map, entrances) = switch_entrance(map, entrance);
    let reachable_map = _all_requirements(&map, &entrances);

    let all_keys = find_all_keys(&map);
    let mut cache = HashMap::new();
    let res = find_from4(
        &map,
        &reachable_map,
        all_keys,
        [
            From::Entrance(entrances[0]),
            From::Entrance(entrances[1]),
            From::Entrance(entrances[2]),
            From::Entrance(entrances[3]),
        ],
        all_keys,
        &mut cache,
    )
    .ok_or(err_msg("empty (a)?"));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_bits() {
        let kb = KeyBits::default();
        assert_eq!(kb.display(), "");
        assert!(!kb.contains('a'));

        let kb1 = kb + 'a' + 'b';
        assert_eq!(kb1.display(), "ab");
        assert!(kb1.contains('a'));

        let kb2 = kb1 - 'a';
        assert_eq!(kb2.display(), "b");
        assert!(kb2.contains('b'));
        assert!(!kb2.contains('a'));

        assert_eq!((kb1 - kb), kb1);
        assert_eq!((kb2 - kb), kb2);
        assert_eq!((kb1 - kb2), vec!['a'].into_iter().collect());
        assert_eq!((kb2 - kb1), kb);

        assert_eq!((kb2 + kb1), kb1);
        assert_eq!(
            vec!['a', 'b'].into_iter().collect::<KeyBits>()
                + vec!['a', 'c'].into_iter().collect::<KeyBits>(),
            vec!['a', 'b', 'c'].into_iter().collect::<KeyBits>()
        )
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
            72
        );
        Ok(())
    }
}
