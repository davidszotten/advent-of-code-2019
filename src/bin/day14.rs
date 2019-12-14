use aoc2019::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::Regex;
// use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(_input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Ingredient {
    name: String,
    amount: usize,
}

impl FromStr for Ingredient {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                // 13 CA
                Regex::new(r"(\d+) (\w+)")
                    .expect("regex create");
        }

        let caps = RE.captures(s).expect("regex match");
        Ok(Self {
            amount: caps[1].parse().expect("regex match 1"),
            name: caps[2].parse().expect("regex match 2"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Recipe {
    makes: Ingredient,
    requires: Vec<Ingredient>,
}

impl FromStr for Recipe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE1: Regex =
                // 3 A, 4 B => 1 AB
                Regex::new(r"(?P<ingredients>((?P<ingredient>[^,]+)(, )?)+) => (?P<result>.*)")
                    .expect("regex create");
        }
        lazy_static! {
            static ref RE2: Regex =
                // 3 A, 4 B
                Regex::new(r"(?P<ingredient>[^,]+)(, )?")
                    .expect("regex create");
        }

        let caps1 = RE1.captures(s).expect("regex match");

        let ingredients: Vec<_> = RE2
            .captures_iter(
                caps1
                    .name("ingredients")
                    .ok_or(err_msg("ingredients parse fail"))?
                    .as_str(),
            )
            .map(|c| {
                c.name("ingredient")
                    .expect("ingredient parse fail")
                    .as_str()
                    .parse::<Ingredient>()
                    .expect("can't make ingredient")
            })
            .collect();
        // dbg!(&caps1);
        let result = caps1
            .name("result")
            .ok_or(err_msg("result parse fail"))?
            .as_str()
            .parse::<Ingredient>()?;

        Ok(Recipe {
            makes: result,
            requires: ingredients,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() -> Result<()> {
        assert_eq!(
            "7 A, 1 E => 1 FUEL".parse::<Recipe>()?,
            Recipe {
                makes: Ingredient {
                    name: "FUEL".into(),
                    amount: 1
                },
                requires: vec![
                    Ingredient {
                        name: "A".into(),
                        amount: 7
                    },
                    Ingredient {
                        name: "E".into(),
                        amount: 1
                    }
                ]
            }
        );
        Ok(())
    }

    //     #[test]
    //     fn test_parse() -> Result<()> {
    //         assert_eq!(parse("10 ORE => 10 A
    // 1 ORE => 1 B
    // 7 A, 1 B => 1 C
    // 7 A, 1 C => 1 D
    // 7 A, 1 D => 1 E
    // 7 A, 1 E => 1 FUEL")?, [()];
    //         Ok(())
    //     }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("")?, 0);
        Ok(())
    }
}
