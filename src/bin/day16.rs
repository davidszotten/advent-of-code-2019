use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn parse(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c as i64 - '0' as i64)
        .filter(|&d| d >= 0 && d <= 9)
        .collect()
}

// 1 1 1 0 0 0 -1 -1 -1 0 0 0 1 1 1 0 0 0

fn pattern(repeat: usize, n: usize) -> i64 {
    let repeat = repeat as i64;
    let n = n as i64 + 1;
    let n = n % (repeat * 4);
    let group = n / repeat;
    match group {
        0 | 2 => 0,
        1 => 1,
        3 => -1,
        _ => {
            unreachable!("invalid group");
        }
    }
}

fn value(digits: &[i64], n: usize) -> i64 {
    digits[n % digits.len()]
}

fn round(digits: Vec<i64>) -> Vec<i64> {
    let mut result = Vec::with_capacity(digits.len());
    for idx in 0..digits.len() {
        // let mut pattern = RepeatEach::new(idx + 1);
        // pattern.next().unwrap();
        let sum: i64 = digits
            .iter()
            .enumerate()
            .map(|(i, d)| d * pattern(idx + 1, i))
            .sum();
        result.push((sum % 10).abs());
    }
    result
}

fn part1(input: &str) -> Result<usize> {
    let mut digits = parse(input);
    for _ in 0..100 {
        digits = round(digits);
    }
    Ok(to_number(&digits[..8]))
}

fn to_number(digits: &[i64]) -> usize {
    let mut res = 0;
    for &digit in digits {
        res = res * 10 + digit as usize
    }
    res
}

fn round2(digits: Vec<i64>) -> Vec<i64> {
    let len = digits.len() as usize;
    let mut result = vec![0; len];
    let mut partial = 0;
    for pos in (0..len).rev() {
        let prev = (pos, partial, digits[pos]);
        partial = (partial + digits[pos]) % 10;
        if partial < 0 {
            unreachable!("{:?}", prev);
        }
        result[pos] = partial;
    }
    result
}

fn part2(input: &str) -> Result<usize> {
    let digits = parse(input);
    let start = to_number(&digits[..7]);
    let total = digits.len() * 10_000;
    let mut tail_digits: Vec<i64> = (start..total).map(|i| value(&digits, i)).collect();
    for _ in 0..100 {
        tail_digits = round2(tail_digits);
    }
    Ok(to_number(&tail_digits[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("1234"), vec![1, 2, 3, 4]);
    }
    // #[test]
    // fn test_repeat_each() {
    // assert_eq!(
    // RepeatEach::new(2).take(12).collect::<Vec<_>>(),
    // vec![0, 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1]
    // );
    // }

    #[test]
    fn test_round() -> Result<()> {
        assert_eq!(round(parse("12345678")), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        Ok(())
    }

    #[test]
    fn test_pattern() {
        assert_eq!(
            (0..17).map(|i| pattern(3, i)).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1]
        )
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("80871224585914546619083218645595")?, 24176176);
        assert_eq!(part1("19617804207202209144916044189917")?, 73745418);
        assert_eq!(part1("69317163492948606335995924319873")?, 52432133);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("03036732577212944063491565474664")?, 84462026);
        assert_eq!(part2("02935109699940807407585447034323")?, 78725270);
        assert_eq!(part2("03081770884921959731165446850517")?, 53553731);
        Ok(())
    }
}
