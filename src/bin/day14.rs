use aoc2019::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn make(available: &mut HashMap<String, i64>, recipe_map: &HashMap<String, Recipe>) -> Result<()> {
    Ok(loop {
        let needed: Vec<(String, i64)> = available
            .iter()
            .filter(|&(name, &amount)| name != "ORE" && amount < 0)
            .map(|(name, amount)| (name.clone(), -*amount))
            .collect();
        if needed.len() == 0 {
            break;
        }
        for (ingredient, amount) in needed {
            let recipe = recipe_map
                .get(&ingredient)
                .ok_or(err_msg("ingredient not found"))?;
            let multiple = max(amount / recipe.makes.amount, 1);
            *available.entry(ingredient).or_insert(0) += recipe.makes.amount * multiple;
            for ingredient in recipe.requires.iter() {
                *available.entry(ingredient.name.clone()).or_insert(0) -=
                    ingredient.amount * multiple;
            }
        }
    })
}

fn part1(input: &str) -> Result<i64> {
    let recipe_map = parse(input)?;
    let mut available: HashMap<String, i64> = HashMap::new();
    available.insert("FUEL".into(), -1);
    make(&mut available, &recipe_map)?;
    available.get("ORE").ok_or(err_msg("no ore?")).map(|&n| -n)
}

fn part2(input: &str) -> Result<i64> {
    let recipe_map = parse(input)?;
    let mut available: HashMap<String, i64> = HashMap::new();
    available.insert("ORE".into(), 1000000000000);
    let mut fuel = 0;
    let mut scale = 1;
    loop {
        let prev = available.clone();
        *available.entry("FUEL".into()).or_insert(0) -= scale;
        make(&mut available, &recipe_map)?;

        let ore = *available.get("ORE").ok_or(err_msg("no ore?"))?;
        if ore < 0 {
            if scale == 1 {
                break Ok(fuel);
            } else {
                available = prev;
                scale /= 10;
            }
        } else {
            fuel += scale;
            scale *= 10;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Ingredient {
    name: String,
    amount: i64,
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

        let caps = RE.captures(s).ok_or(err_msg("no match"))?;
        Ok(Self {
            amount: caps[1].parse()?,
            name: caps[2].parse()?,
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

        let caps1 = RE1.captures(s).ok_or(err_msg("no regex match"))?;

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

fn parse(input: &str) -> Result<HashMap<String, Recipe>> {
    let recipes = input
        .split("\n")
        .map(|row| row.parse::<Recipe>())
        .collect::<Result<Vec<_>>>()?;
    Ok(recipes
        .iter()
        .map(|recipe| (recipe.makes.name.clone(), recipe.clone()))
        .collect())
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

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            )?,
            31
        );
        Ok(())
    }
    #[test]
    fn test_part1b() -> Result<()> {
        assert_eq!(
            part1(
                "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL"
            )?,
            165
        );
        Ok(())
    }

    #[test]
    fn test_part1d() -> Result<()> {
        assert_eq!(
            part1(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF"
            )?,
            180697
        );
        Ok(())
    }

    #[test]
    fn test_part1c() -> Result<()> {
        assert_eq!(
            part1(
                "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX"
            )?,
            2210736
        );
        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<()> {
        assert_eq!(
            part2(
                "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )?,
            82892753
        );
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF"
            )?,
            5586022
        );
        Ok(())
    }

    #[test]
    fn test_part2c() -> Result<()> {
        assert_eq!(
            part2(
                "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX"
            )?,
            460664
        );
        Ok(())
    }
}
