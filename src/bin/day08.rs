use aoc2019::{dispatch, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn checksum(input: &str, size: usize) -> Result<i32> {
    let mut fewest0 = size;
    let mut best_count = 0;
    for layer in input.chars().collect::<Vec<_>>().chunks(size) {
        let mut counts = HashMap::new();
        for digit in layer {
            *counts.entry(digit).or_insert(0) += 1;
        }
        let get_count = |d| *counts.get(&d).unwrap_or(&0);
        let zeros = get_count('0');
        if zeros < fewest0 {
            fewest0 = zeros;
            best_count = (get_count('1') * get_count('2')) as i32;
        }
    }

    Ok(best_count)
}

fn part1(input: &str) -> Result<i32> {
    checksum(input, 25 * 6)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Transparent,
    White,
    Black,
}

fn part2(input: &str) -> Result<i32> {
    use Pixel::*;
    let size = 25 * 6;
    let mut output = vec![Transparent; size];
    for layer in input.chars().collect::<Vec<_>>().chunks(size) {
        for (index, digit) in layer.iter().enumerate() {
            if output[index] != Transparent {
                continue;
            }
            match digit {
                '0' => output[index] = Black,
                '1' => output[index] = White,
                '2' => {}
                c => unreachable!("unexpected digit: {:?}", c),
            }
        }
    }
    for row in output.chunks(25) {
        for c in row.iter().map(|p| match p {
            Transparent => unreachable!("transparent pixel"),
            White => '#',
            Black => ' ',
        }) {
            print!("{}", c)
        }
        print!("\n");
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(checksum("123456789012", 2 * 3)?, 1);
        Ok(())
    }
}
