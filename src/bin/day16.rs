use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

const BASE_PATTERN: [i16; 4] = [0, 1, 0, -1];

struct RepeatEach {
    times: usize,
    pos: usize,
    el: Option<i16>,
    iterator: Box<dyn Iterator<Item = i16>>,
}

impl RepeatEach {
    fn new(times: usize) -> Self {
        RepeatEach {
            times,
            pos: 0,
            el: None,
            iterator: Box::new(BASE_PATTERN.iter().map(|&n| n).cycle()),
        }
    }
}
impl Iterator for RepeatEach {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            self.el = self.iterator.next();
        }
        self.pos = (self.pos + 1) % self.times;
        self.el
    }
}

fn parse(input: &str) -> Vec<i16> {
    input.chars().map(|c| c as i16 - '0' as i16).collect()
}

fn round(digits: Vec<i16>) -> Vec<i16> {
    let mut result = vec![];
    for idx in 0..digits.len() {
        let mut pattern = RepeatEach::new(idx + 1);
        pattern.next().unwrap();
        let sum: i16 = digits.iter().zip(pattern).map(|(d, p)| d * p).sum();
        result.push((sum % 10).abs());
    }
    result
}

fn part1(input: &str) -> Result<Vec<i16>> {
    let mut digits = parse(input);
    for _ in 0..100 {
        digits = round(digits);
    }
    Ok(digits[..8].iter().map(|&n| n).collect())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("1234"), vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_repeat_each() {
        assert_eq!(
            RepeatEach::new(2).take(12).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1]
        );
    }

    #[test]
    fn test_round() -> Result<()> {
        assert_eq!(round(parse("12345678")), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1("80871224585914546619083218645595")?,
            parse("24176176")
        );
        assert_eq!(
            part1("19617804207202209144916044189917")?,
            parse("73745418")
        );
        assert_eq!(
            part1("69317163492948606335995924319873")?,
            parse("52432133")
        );
        Ok(())
    }
}
