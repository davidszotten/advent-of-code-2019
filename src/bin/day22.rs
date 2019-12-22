use aoc2019::{dispatch, Result};
use failure::{bail, err_msg, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Deal(usize),
}

type CardInt = u32;

impl Shuffle {
    fn apply(&self, cards: Vec<CardInt>) -> Vec<CardInt> {
        let mut new = cards.clone();
        match *self {
            Shuffle::NewStack => {
                new.reverse();
            }
            Shuffle::Cut(amount) if amount >= 0 => {
                new.rotate_left(amount as usize);
            }
            Shuffle::Cut(amount) if amount < 0 => {
                new.rotate_right(amount.abs() as usize);
            }
            Shuffle::Cut(_) => unreachable!(),
            Shuffle::Deal(amount) => {
                for pos in 0..cards.len() {
                    new[(pos * amount) % cards.len()] = cards[pos];
                }
            }
        }
        new
    }
}

impl FromStr for Shuffle {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let new = "deal into new stack";
        let cut = "cut ";
        let deal = "deal with increment ";
        if value.len() >= new.len() && &value[..new.len()] == new {
            return Ok(Shuffle::NewStack);
        }
        if value.len() >= cut.len() && &value[..cut.len()] == cut {
            return Ok(Shuffle::Cut(value[cut.len()..].parse()?));
        }
        if value.len() >= deal.len() && &value[..deal.len()] == deal {
            return Ok(Shuffle::Deal(value[deal.len()..].parse()?));
        }

        bail!("can't parse as shuffle");
    }
}

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn run(shuffles: Vec<Shuffle>, size: CardInt) -> Vec<CardInt> {
    let mut cards: Vec<CardInt> = (0..size).collect();
    for shuffle in shuffles {
        cards = shuffle.apply(cards);
    }
    cards
}

fn parse(input: &str) -> Result<Vec<Shuffle>> {
    input
        .split('\n')
        .map(|l| l.parse())
        .collect::<Result<Vec<Shuffle>>>()
}

fn part1(input: &str) -> Result<usize> {
    let shuffles = parse(input)?;
    let cards = run(shuffles, 10007);
    cards
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 2019)
        .map(|(i, _)| i)
        .next()
        .ok_or(err_msg("not found"))
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!("deal into new stack".parse::<Shuffle>()?, Shuffle::NewStack);
        assert_eq!("cut -2".parse::<Shuffle>()?, Shuffle::Cut(-2));
        assert_eq!(
            "deal with increment 7".parse::<Shuffle>()?,
            Shuffle::Deal(7)
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let shuffles = parse(
            "deal with increment 7
deal into new stack
deal into new stack",
        )?;
        assert_eq!(run(shuffles, 10), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        Ok(())
    }
}
