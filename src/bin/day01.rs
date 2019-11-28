use aoc2019::{dispatch, Result};
use std::collections::HashSet;

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .sum())
}

fn part2(input: &str) -> Result<i32> {
    let mut freq = 0;
    let mut seen = HashSet::<i32>::new();
    seen.insert(freq);
    for value in input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>()
        .iter()
        .cycle()
    {
        freq += value;
        if seen.contains(&freq) {
            return Ok(freq);
        }
        seen.insert(freq);
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&"+1, +1, +1".replace(", ", "\n"))?, 3);
        assert_eq!(part1(&"+1, +1, -2".replace(", ", "\n"))?, 0);
        assert_eq!(part1(&"-1, -2, -3".replace(", ", "\n"))?, -6);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&"+1, -1".replace(", ", "\n"))?, 0);
        assert_eq!(part2(&"+3, +3, +4, -2, -4".replace(", ", "\n"))?, 10);
        assert_eq!(part2(&"-6, +3, +8, +5, -6".replace(", ", "\n"))?, 5);
        assert_eq!(part2(&"+7, +7, -2, -7, -4".replace(", ", "\n"))?, 14);
        Ok(())
    }
}
