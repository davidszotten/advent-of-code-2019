use aoc2019::{dispatch, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

const SIZE: i8 = 5;

fn idx2coor(idx: usize) -> (i8, i8) {
    let idx = idx as i8;
    (idx % SIZE, idx / SIZE)
}

fn coor2idx(coor: (i8, i8)) -> usize {
    (coor.1 * SIZE + coor.0) as usize
}

fn count(cells: &[bool], coor: (i8, i8)) -> usize {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|d| (d.0 + coor.0, d.1 + coor.1))
        .filter(|c| c.0 >= 0 && c.0 < SIZE && c.1 >= 0 && c.1 < SIZE)
        .filter(|&c| cells[coor2idx(c)])
        .count()
}

fn next(cells: &[bool]) -> Vec<bool> {
    cells
        .iter()
        .enumerate()
        .map(|(idx, &c)| match (c, count(cells, idx2coor(idx))) {
            (true, 1) => true,
            (true, _) => false,
            (false, 1) => true,
            (false, 2) => true,
            (b, _) => b,
        })
        .collect()
}

fn _draw(cells: &[bool]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!(
                "{}",
                match cells[coor2idx((x, y))] {
                    false => '.',
                    true => '#',
                }
            );
        }
        print!("\n");
    }
    print!("\n");
}

fn rating(cells: &[bool]) -> u32 {
    let mut sum = 0;
    for (idx, &val) in cells.iter().enumerate() {
        if val {
            sum += 1 << idx;
        }
    }
    sum
}

fn parse(input: &str) -> Vec<bool> {
    input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect()
}

fn part1(input: &str) -> Result<u32> {
    let mut cells = parse(input);
    /* draw(&cells); */
    let mut seen = HashSet::new();
    loop {
        let n = rating(&cells);
        if seen.contains(&n) {
            return Ok(n);
        }
        seen.insert(n);
        cells = next(&cells);
    }
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(coor2idx((0, 0)), 0);
        assert_eq!(coor2idx((2, 2)), 12);
        assert_eq!(coor2idx((4, 4)), 24);
        assert_eq!(coor2idx(idx2coor(0)), 0);
        assert_eq!(coor2idx(idx2coor(24)), 24);
        assert_eq!(idx2coor(coor2idx((0, 0))), (0, 0));
        assert_eq!(idx2coor(coor2idx((1, 2))), (1, 2));
        assert_eq!(idx2coor(coor2idx((4, 4))), (4, 4));
    }

    #[test]
    fn test_count() {
        let cells = parse(
            "\
#....
.....
...#.
..#.#
...#.",
        );
        assert_eq!(count(&cells, (0, 0)), 0);
        assert_eq!(count(&cells, (0, 1)), 1);
        assert_eq!(count(&cells, (1, 0)), 1);
        assert_eq!(count(&cells, (3, 3)), 4);
    }

    #[test]
    fn test_next() {
        let cells = parse(
            "\
....#
#..#.
#..##
..#..
#....",
        );
        let expected = parse(
            "\
#..#.
####.
###.#
##.##
.##..",
        );
        assert_eq!(count(&cells, (0, 4)), 0);
        draw(&next(&cells));
        assert_eq!(next(&cells), expected);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "\
....#
#..#.
#..##
..#..
#...."
            )?,
            2129920
        );
        Ok(())
    }
}
