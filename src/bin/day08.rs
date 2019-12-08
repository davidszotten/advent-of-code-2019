use aoc2019::{dispatch, Result};
use failure::err_msg;
use std::collections::HashMap;
use std::io::Write;
use std::str;

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

fn write_image(input: &str, width: usize, height: usize) -> Result<String> {
    let mut buf = Vec::new();
    use Pixel::*;
    let size = width * height;
    let mut output = vec![Transparent; size];
    for layer in input.chars().collect::<Vec<_>>().chunks(size) {
        for (digit, pixel) in layer.iter().zip(output.iter_mut()) {
            if *pixel != Transparent {
                continue;
            }
            match digit {
                '0' => *pixel = Black,
                '1' => *pixel = White,
                '2' => {}
                c => unreachable!("unexpected digit: {:?}", c),
            }
        }
    }
    for row in output.chunks(width) {
        for c in row.iter().map(|p| match p {
            Transparent => unreachable!("transparent pixel"),
            White => '#',
            Black => ' ',
        }) {
            write!(&mut buf, "{}", c)?;
        }
        write!(&mut buf, "\n")?;
    }
    str::from_utf8(&buf)
        .map_err(|_| err_msg("Failed to convert to string"))
        .map(|s| s.to_string())
}

fn part2(input: &str) -> Result<i32> {
    print!("{}", write_image(input, 25, 6)?);

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

    #[test]
    fn test_write_image() -> Result<()> {
        assert_eq!(write_image("0222112222120000", 2, 2)?, " #\n# \n");
        Ok(())
    }
}
