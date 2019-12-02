use aoc2019::{dispatch, Result};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .map(|x| x / 3 - 2)
        .sum())
}

fn recurse_fuel(mass: i32) -> i32 {
    let mut sum = 0;
    let mut fuel = mass / 3 - 2;
    while fuel > 0 {
        sum += fuel;
        fuel = fuel / 3 - 2;
    }
    sum
}

fn part2(input: &str) -> Result<i32> {
    Ok(input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .map(recurse_fuel)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&"12".replace(", ", "\n"))?, 2);
        assert_eq!(part1(&"14".replace(", ", "\n"))?, 2);
        assert_eq!(part1(&"1969".replace(", ", "\n"))?, 654);
        assert_eq!(part1(&"100756".replace(", ", "\n"))?, 33583);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&"14".replace(", ", "\n"))?, 2);
        assert_eq!(part2(&"1969".replace(", ", "\n"))?, 966);
        assert_eq!(part2(&"100756".replace(", ", "\n"))?, 50346);
        Ok(())
    }
}
