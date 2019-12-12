use aoc2019::{dispatch, Result};
use failure::bail;
use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn coordinates(input: &str) -> Vec<(i32, i32)> {
    let mut row = 0;
    let mut col = 0;
    let mut result = vec![];
    for c in input.chars() {
        match c {
            '#' => result.push((col, row)),
            '.' => {}
            '\n' => {
                row += 1;
                col = -1;
            }
            c => unreachable!("unexpected char {}", c),
        }
        col += 1;
    }
    result
}

type Coor = (i32, i32);

fn visibility(station: Coor, asteroids: &[Coor]) -> HashMap<Coor, bool> {
    let mut grouped = HashMap::new();

    for &(x, y) in asteroids.iter() {
        let dx = x - station.0;
        let dy = y - station.1;
        if (dx, dy) == (0, 0) {
            continue;
        }
        let d = gcd(dx, dy);
        let key = (dx / d, dy / d);
        let val: Coor = (x, y);
        grouped.entry(key).or_insert(vec![]).push((d, val));
    }

    let mut result = HashMap::new();
    for (_key, entries) in grouped {
        let mut distances: Vec<_> = entries.iter().map(|(d, _)| d).collect();
        distances.sort();
        let &smallest = distances[0];

        for (d, val) in entries {
            result.insert(val, d == smallest);
        }
    }

    result
}

fn find_max(input: &str) -> (usize, i32, i32) {
    let mut counts = vec![];

    let asteroids = coordinates(input);
    for (x, y) in asteroids.iter() {
        let mut visible = HashSet::new();
        for (nx, ny) in asteroids.iter() {
            let dx = nx - x;
            let dy = ny - y;
            if (dx, dy) == (0, 0) {
                continue;
            }
            let d = gcd(dx, dy);
            visible.insert((dx / d, dy / d));
        }
        counts.push((visible.len(), *x, *y));
    }
    counts.sort_by_key(|&t| t.0);

    counts[counts.len() - 1]
}

fn part1(input: &str) -> Result<usize> {
    Ok(find_max(input).0)
}

fn part2(input: &str) -> Result<i32> {
    let (_, station_x, station_y) = find_max(input);
    let asteroids = coordinates(input);
    let mut aset: HashSet<_> = asteroids.iter().clone().collect();
    aset.remove(&(station_x, station_y));
    let mut n = 1;

    while aset.len() > 0 {
        let remaining: Vec<Coor> = aset.iter().map(|&(a, b)| (*a, *b)).collect();
        let visibility = visibility((station_x, station_y), &remaining[..]);
        let mut visible = vec![];
        for &(x, y) in &remaining {
            let dx = x - station_x;
            let dy = y - station_y;
            if (dx, dy) == (0, 0) {
                continue;
            }
            if !visibility.get(&(x, y)).expect("visibility unknown") {
                continue;
            }
            let quadrant = match (dx.signum(), dy.signum()) {
                (0, -1) => 0,
                (1, -1) => 1,
                (1, 0) => 2,
                (1, 1) => 3,
                (0, 1) => 4,
                (-1, 1) => 5,
                (-1, 0) => 6,
                (-1, -1) => 7,
                _ => unreachable!("bad signum?"),
            };
            visible.push((quadrant, dx, dy, x, y));
        }
        visible.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => (a.2 * b.1).cmp(&(a.1 * b.2)),
            Ordering::Greater => Ordering::Greater,
        });
        let visible: Vec<_> = visible.iter().map(|&(_q, _dx, _dy, x, y)| (x, y)).collect();
        for entry in visible.into_iter() {
            if n == 200 {
                return Ok(entry.0 * 100 + entry.1);
            }
            aset.remove(&entry);
            n += 1;
        }
    }
    bail!("Didn't get to 200");
}

// y1/x1 > y2/x2 -> y1x2 > y2x1
//  -1,-1 | 1,-1
//  --------------
//  -1,1  | 1,1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        assert_eq!(
            coordinates(
                ".#..#
.....
#####
....#
...##"
            ),
            vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                ".#..#
.....
#####
....#
...##"
            )?,
            8
        );
        Ok(())
    }

    #[test]
    fn test_find_max1() {
        assert_eq!(
            find_max(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ),
            (33, 5, 8)
        );
    }

    #[test]
    fn test_find_max2() {
        assert_eq!(
            find_max(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            ),
            (35, 1, 2)
        );
    }

    #[test]
    fn test_find_max3() {
        assert_eq!(
            find_max(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ),
            (210, 11, 13)
        );
    }

    #[test]
    fn test_part2_init() {
        assert_eq!(
            find_max(
                ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"
            ),
            (30, 8, 3)
        );
    }

    //     #[test]
    //     fn test_part2() -> Result<()> {
    //         assert_eq!(
    //             part2(
    //                 ".#....#####...#..
    // ##...##.#####..##
    // ##...#...#.#####.
    // ..#.....#...###..
    // ..#.#.....#....##"
    //             )?,
    //             0
    //         );
    //         Ok(())
    //     }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            )?,
            802
        );
        Ok(())
    }

    #[test]
    fn test_visibility() {
        assert_eq!(
            visibility(
                (0, 2),
                &coordinates(
                    ".#..#
.....
#####
....#
...##"
                )
            ),
            [
                ((1, 0), true),
                ((4, 0), true),
                // (0, 2),
                ((1, 2), true),
                ((2, 2), false),
                ((3, 2), false),
                ((4, 2), false),
                ((4, 3), true),
                ((3, 4), true),
                ((4, 4), true),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<Coor, bool>>()
        )
    }
}
