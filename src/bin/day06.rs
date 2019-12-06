use aoc2019::{dispatch, Result};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn count(parents: &HashMap<&str, &str>, body: &str) -> i32 {
    match parents.get(body) {
        Some(parent) => count(&parents, parent.clone()) + 1,
        None => 0,
    }
}

fn parse(input: &str) -> HashMap<&str, &str> {
    let orbits: Vec<_> = input
        .split('\n')
        .map(|l| {
            let mut pair = l.split(')');
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect();

    let mut parents = HashMap::new();
    for (parent, child) in orbits {
        if let Some(_other) = parents.get(child) {
            panic!("multiple 1");
        }
        parents.insert(child, parent);
    }
    parents
}

fn part1(input: &str) -> Result<i32> {
    let parents = parse(input);
    let sum = parents.keys().map(|&body| count(&parents, body)).sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<i32> {
    let parents = parse(input);
    let mut children_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, value) in parents.iter() {
        children_map.entry(&value).or_insert(vec![]).push(key);
    }
    let start = parents.get("YOU").unwrap();
    let end = parents.get("SAN").unwrap();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start, 0));
    seen.insert(start);
    while let Some((next, distance)) = queue.pop_front() {
        seen.insert(next);
        if next == end {
            return Ok(distance);
        }
        if let Some(parent) = parents.get(next) {
            if !seen.contains(parent) {
                queue.push_back((parent, distance + 1));
            }
        }
        if let Some(chilren) = children_map.get(next) {
            for child in chilren.iter() {
                if !seen.contains(child) {
                    queue.push_back((child, distance + 1));
                }
            }
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            )?,
            42
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            )?,
            4
        );
        Ok(())
    }
}
