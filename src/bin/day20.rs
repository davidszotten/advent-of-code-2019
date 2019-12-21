use aoc2019::coor::Coor;
use aoc2019::{dispatch, Result};
use failure::err_msg;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Input {
    Open,
    Wall,
    Label(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Label {
    Outside(String),
    Inside(String),
}

impl Label {
    fn label(&self) -> &str {
        use Label::*;
        match self {
            Inside(label) => label,
            Outside(label) => label,
        }
    }
    fn level_diff(&self, other: &Label) -> i64 {
        use Label::*;
        match (self, other) {
            (Inside(_), Outside(_)) => 1,
            (Outside(_), Inside(_)) => -1,
            _ => 0,
        }
    }
}

fn parse(input: &str) -> (HashMap<Coor, Tile>, HashMap<Coor, Label>) {
    let mut text_map = HashMap::new();
    let mut pos = Coor::new(0, 0);
    for c in input.chars() {
        if c == '\n' {
            pos = Coor::new(0, pos.y + 1);
        } else {
            if let Some(tile) = match c {
                '#' => Some(Input::Wall),
                '.' => Some(Input::Open),
                ' ' => None,
                c if 'A' <= c && c <= 'Z' => Some(Input::Label(c)),
                c => unreachable!("invalid char: {}", c),
            } {
                text_map.insert(pos, tile);
            }
            pos += Coor::new(1, 0);
        }
    }

    let mut coorsx = text_map.keys().collect::<Vec<_>>();
    let mut coorsy = text_map.keys().collect::<Vec<_>>();
    coorsx.sort_by_key(|c| -c.x);
    coorsy.sort_by_key(|c| -c.y);
    let max = Coor::new(coorsx[0].x, coorsy[0].y);

    let mut map = HashMap::new();
    let mut labels = HashMap::new();
    for (&coor, &tile) in &text_map {
        match tile {
            Input::Label(c) => {
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
                    if let Some(Input::Label(d)) = text_map.get(a) {
                        if let Some(Input::Open) = text_map.get(b) {
                            let label = if *order == 0 { [c, *d] } else { [*d, c] }
                                .iter()
                                .collect::<String>();
                            if a.x == 0
                                || coor.x == 0
                                || a.x == max.x
                                || coor.x == max.x
                                || a.y == 0
                                || coor.y == 0
                                || a.y == max.y
                                || coor.y == max.y
                            {
                                labels.insert(b.clone(), Label::Outside(label));
                            } else {
                                labels.insert(b.clone(), Label::Inside(label));
                            }
                        }
                    }
                }
            }
            Input::Open => {
                map.insert(coor, Tile::Open);
            }
            Input::Wall => {
                map.insert(coor, Tile::Wall);
            }
        }
    }
    (map, labels)
}

fn find_single_label(labels: &HashMap<Coor, Label>, label: &str) -> Result<Coor> {
    labels
        .iter()
        .filter(|(_, l)| l.label() == label)
        .map(|(coor, _)| *coor)
        .next()
        .ok_or(err_msg("label not found"))
}

fn find_other_label(labels: &HashMap<Coor, Label>, coor: &Coor) -> Option<(Coor, i64)> {
    let label = labels.get(coor)?;
    labels
        .iter()
        .filter(|(c, l)| l.label() == label.label() && c != &coor)
        .map(|(coor, l)| (*coor, label.level_diff(l)))
        .next()
}

fn part1(input: &str) -> Result<i32> {
    let (map, labels) = parse(input);
    let start = find_single_label(&labels, "AA")?;
    let end = find_single_label(&labels, "ZZ")?;

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 0));

    while let Some((pos, level, distance)) = queue.pop_front() {
        if pos == end {
            return Ok(distance);
        }
        seen.insert((pos, level));

        for mv in [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ]
        .iter()
        {
            let next = pos + *mv;
            match map.get(&next) {
                None => continue,
                Some(Tile::Wall) => continue,
                Some(Tile::Open) => {}
            }
            if seen.contains(&(next, level)) {
                continue;
            }
            queue.push_back((next, level, distance + 1));
        }
        if let Some((bridged, level_diff)) = find_other_label(&labels, &pos) {
            let new_level = level + level_diff;
            if !seen.contains(&(bridged, new_level)) {
                queue.push_back((bridged, new_level, distance + 1));
            }
        }
    }

    Ok(0)
}

fn part2(input: &str) -> Result<i32> {
    let (map, labels) = parse(input);
    let start = find_single_label(&labels, "AA")?;
    let end = find_single_label(&labels, "ZZ")?;

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 0));

    while let Some((pos, level, distance)) = queue.pop_front() {
        if pos == end && level == 0 {
            return Ok(distance);
        }
        seen.insert((pos, level));

        for mv in [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ]
        .iter()
        {
            let next = pos + *mv;
            match map.get(&next) {
                None => continue,
                Some(Tile::Wall) => continue,
                Some(Tile::Open) => {}
            }
            if seen.contains(&(next, level)) {
                continue;
            }
            queue.push_back((next, level, distance + 1));
        }
        if let Some((bridged, level_diff)) = find_other_label(&labels, &pos) {
            let new_level = level + level_diff;
            if new_level >= 0 && !seen.contains(&(bridged, new_level)) {
                queue.push_back((bridged, new_level, distance + 1));
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
                "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "
            )?,
            23
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "
            )?,
            26
        );
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     "
            )?,
            396
        );
        Ok(())
    }
}
