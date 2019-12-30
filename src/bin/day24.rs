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
                if x == 2 && y == 2 {
                    '?'
                } else {
                    match cells[coor2idx((x, y))] {
                        false => '.',
                        true => '#',
                    }
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
            '.' | '?' => Some(false),
            '#' => Some(true),
            '\n' => None,
            c => panic!("Unexpected char: {}", c),
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

type Coor3 = (i8, i8, i64);
// type Vec3 = Vec<Vec<Vec<bool>>>;

struct Cells {
    data: Vec<Vec<bool>>,
}
impl Cells {
    // fn new(levels: usize) -> Self {
    //     let mut data = vec![];
    //     for _ in 0..levels {
    //         data.push(vec![false; 25])
    //     }
    //     Self { data }
    // }

    fn get(&self, x: i8, y: i8, level: i64) -> bool {
        if level < 0 || level >= self.data.len() as i64 {
            return false;
        }
        if x < 0 || x >= SIZE || y < 0 || y >= SIZE {
            panic!("Bad (x,y): ({},{})", x, y);
        }
        self.data[level as usize][coor2idx((x, y))]
    }

    fn count(&self) -> usize {
        self.data
            .iter()
            .map(|level| level.iter().filter(|b| **b).count())
            .sum()
    }

    fn _draw(&self) {
        for level in &self.data {
            // if level.iter().any(|b| *b) {
            _draw(&level);
            // }
        }
    }
}

fn count_neighbours_at(cells: &Cells, coor: Coor3, from: Coor3) -> usize {
    let single = |b| if b { 1 } else { 0 };
    if coor.0 < 0 {
        return single(cells.get(1, 2, coor.2 - 1));
    }
    if coor.0 >= SIZE {
        return single(cells.get(3, 2, coor.2 - 1));
    }

    if coor.1 < 0 {
        return single(cells.get(2, 1, coor.2 - 1));
    }
    if coor.1 >= SIZE {
        return single(cells.get(2, 3, coor.2 - 1));
    }

    if coor.0 == 2 && coor.1 == 2 {
        return match (from.0, from.1) {
            (1, 2) => (0..5).map(|i| single(cells.get(0, i, coor.2 + 1))).sum(),
            (2, 1) => (0..5).map(|i| single(cells.get(i, 0, coor.2 + 1))).sum(),
            (3, 2) => (0..5).map(|i| single(cells.get(4, i, coor.2 + 1))).sum(),
            (2, 3) => (0..5).map(|i| single(cells.get(i, 4, coor.2 + 1))).sum(),
            _ => {
                unreachable!("Invalid neighbour: {:?}, {:?}", coor, from);
            }
        };
    }

    return single(cells.get(coor.0, coor.1, coor.2));
}

fn counti(cells: &Cells, coor: Coor3) -> usize {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|d| (d.0 + coor.0, d.1 + coor.1))
        .map(|c| count_neighbours_at(&cells, (c.0, c.1, coor.2), coor))
        .sum()
}

fn nexti(cells: Cells) -> Cells {
    let mut extended_data = cells.data.clone();
    extended_data.insert(0, vec![false; 25]);
    extended_data.push(vec![false; 25]);
    let extended_cells = Cells {
        data: extended_data.clone(),
    };
    Cells {
        data: extended_data
            .iter()
            .enumerate()
            .map(|(level_idx, level)| {
                level
                    .iter()
                    .enumerate()
                    .map(|(idx, &c)| {
                        let coor2 = idx2coor(idx);
                        if coor2 == (2, 2) {
                            return false;
                        }
                        let count = counti(&extended_cells, (coor2.0, coor2.1, level_idx as i64));
                        let new = match (c, count) {
                            (true, 1) => true,
                            (true, _) => false,
                            (false, 1) => true,
                            (false, 2) => true,
                            (b, _) => b,
                        };
                        new
                    })
                    .collect()
            })
            .collect(),
    }
}

fn part2(input: &str) -> Result<usize> {
    let mut cells = Cells {
        data: vec![parse(input)],
    };
    for _ in 0..200 {
        cells = nexti(cells);
    }
    Ok(cells.count())
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
        _draw(&next(&cells));
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

    #[test]
    fn test_count_neighbours_at_19() {
        let mut middle = vec![false; (SIZE * SIZE) as usize];
        middle[coor2idx((3, 2))] = true;
        middle[coor2idx((2, 3))] = true;
        middle[coor2idx((3, 4))] = true;
        middle[coor2idx((4, 3))] = true;
        let cells = Cells {
            data: vec![
                vec![false; (SIZE * SIZE) as usize],
                middle,
                vec![false; (SIZE * SIZE) as usize],
            ],
        };
        assert_eq!(cells.count(), 4);
        assert_eq!(counti(&cells, (3, 3, 1)), 4);
    }

    #[test]
    fn test_count_neighbours_at_14() {
        let mut middle = vec![false; (SIZE * SIZE) as usize];
        middle[coor2idx((3, 1))] = true;
        middle[coor2idx((3, 3))] = true;
        middle[coor2idx((4, 2))] = true;
        let mut bottom = vec![false; (SIZE * SIZE) as usize];
        for i in 0..5 {
            bottom[coor2idx((4, i))] = true;
        }
        let cells = Cells {
            data: vec![vec![false; (SIZE * SIZE) as usize], middle, bottom],
        };
        assert_eq!(cells.count(), 8);
        assert_eq!(counti(&cells, (3, 2, 1)), 8);
    }

    #[test]
    fn test_counti() {
        let initial = parse(
            "\
....#
#..#.
#.?##
..#..
#....
",
        );
        let mut cells = Cells {
            data: vec![initial],
        };

        let mut extended_data = cells.data.clone();
        extended_data.insert(0, vec![false; 25]);
        extended_data.push(vec![false; 25]);

        assert_eq!(
            counti(
                &Cells {
                    data: extended_data
                },
                (0, 0, 2)
            ),
            0
        );
        for _ in 0..10 {
            cells = nexti(cells);
        }
        // cells._draw();
        assert_eq!(cells.count(), 99);
    }
}
