#![feature(slice_patterns)]

use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

struct Digits {
    number: i32,
}

impl Digits {
    fn new(number: i32) -> Self {
        Digits { number }
    }
}

impl Iterator for Digits {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.number == 0 {
            return None;
        }
        let digit = self.number % 10;
        self.number /= 10;
        Some(digit)
    }
}

fn is_password(number: i32) -> bool {
    let mut found_double = false;
    for win in Digits::new(number).collect::<Vec<_>>().windows(2) {
        let a = win[0];
        let b = win[1];
        found_double |= a == b;
        if a < b {
            return false;
        }
    }
    found_double
}

fn is_password2(number: i32) -> bool {
    let mut digits = [0; 10];
    if Digits::new(number)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|win| (win[0], win[1]))
        .any(|(a, b)| a < b)
    {
        return false;
    }
    for digit in Digits::new(number) {
        digits[digit as usize] += 1;
    }

    digits.iter().any(|&count| count == 2)
}

fn part1(input: &str) -> Result<i32> {
    let start_end: Vec<_> = input
        .split('-')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    Ok((start_end[0]..=start_end[1])
        .filter(|&x| is_password(x))
        .count() as i32)
}

fn part2(input: &str) -> Result<i32> {
    let start_end: Vec<_> = input
        .split('-')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    Ok((start_end[0]..=start_end[1])
        .filter(|&x| is_password2(x))
        .count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_password() {
        assert_eq!(is_password(111111), true);
        assert_eq!(is_password(223450), false);
        assert_eq!(is_password(123789), false);
    }

    #[test]
    fn test_is_password2() {
        assert_eq!(is_password2(112233), true);
        assert_eq!(is_password2(123444), false);
        assert_eq!(is_password2(111122), true);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("123787-123789")?, 1);
        Ok(())
    }
}
