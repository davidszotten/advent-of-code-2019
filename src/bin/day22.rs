use aoc2019::{dispatch, Result};
use failure::{bail, Error};
use std::ops;
use std::str::FromStr;

type CardInt = i64;

fn gcd_extended(a: CardInt, b: CardInt) -> (CardInt, CardInt, CardInt) {
    // (b, x, y)

    if a == 0 {
        return (b, 0, 1);
    }

    let (g, x, y) = gcd_extended(b % a, a);

    return (g, y - (b / a) * x, x);
}

fn mod_inverse(a: CardInt, m: CardInt) -> CardInt {
    let (g, x, _) = gcd_extended(a, m);
    if g != 1 {
        unimplemented!("inverse doesn't exist");
    }
    // m is added to handle negative x
    (x % m + m) % m
}

fn mod_div(a: CardInt, b: CardInt, m: CardInt) -> CardInt {
    mod_mul(a, mod_inverse(b, m), m)
}

fn mod_mul(a: CardInt, b: CardInt, m: CardInt) -> CardInt {
    let mut res = 0;
    let mut a = a % m;
    let mut b = b;
    while b > 0 {
        // If b is odd, add 'a' to result
        if b % 2 == 1 {
            res = (res + a) % m;
        }

        // Multiply 'a' with 2
        a = (a * 2) % m;
        // Divide b by 2
        b /= 2;
    }

    res % m
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Shuffle {
    NewStack,
    Cut(i64),
    Deal(i64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Variable {
    x: CardInt,
    c: CardInt,
    m: CardInt,
}

impl Variable {
    fn new(x: CardInt, c: CardInt, m: CardInt) -> Self {
        Self {
            x: (x + m) % m,
            c: (c + m) % m,
            m,
        }
    }

    fn eval(&self, card: CardInt) -> CardInt {
        (mod_mul(self.x, card, self.m) + self.c + self.m) % self.m
    }

    fn invert(&self) -> Self {
        // y = xc +b -> x = y/c - b/c
        Variable::new(
            mod_div(1, self.x, self.m),
            mod_div(-self.c, self.x, self.m),
            self.m,
        )
    }
}

impl ops::Mul for Variable {
    type Output = Variable;
    fn mul(self, other: Variable) -> Variable {
        // (ax + c)(bx + d) = a(bx+d) +c = abx + ad + c
        assert_eq!(self.m, other.m);
        let m = self.m;
        Variable::new(
            mod_mul(self.x, other.x, m),
            mod_mul(self.x, other.c, m) + self.c,
            m,
        )
    }
}

impl Shuffle {
    fn _apply(&self, cards: Vec<CardInt>) -> Vec<CardInt> {
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
                    new[(pos * amount as usize) % cards.len()] = cards[pos];
                }
            }
        }
        new
    }

    fn apply_to(&self, card: CardInt, len: CardInt) -> CardInt {
        match *self {
            Shuffle::NewStack => (len - 1 - card) % len,
            Shuffle::Cut(amount) => (card - amount + len) % len,
            Shuffle::Deal(amount) => mod_mul(card, amount, len),
        }
    }

    fn apply_to_variable(&self, card: Variable, len: CardInt) -> Variable {
        // v = x + c
        // NewStack: -1 -v  -> -1 -(c+x) -> -x -1 -c ->
        // Cut(amount): v - amount -> x + c -amount
        // Deal(amount): amount * v -> amount *x + amount * c
        match *self {
            Shuffle::NewStack => Variable::new(-card.x, -1 - card.c, len),
            Shuffle::Cut(amount) => Variable::new(card.x, card.c - amount, len),
            Shuffle::Deal(amount) => Variable::new(
                mod_mul(card.x, amount, len),
                mod_mul(card.c, amount, len),
                len,
            ),
        }
    }

    fn _apply_reverse_to(&self, card: CardInt, len: CardInt) -> CardInt {
        match *self {
            Shuffle::NewStack => (len - 1 - card) % len,
            Shuffle::Cut(amount) => card + amount % len,
            Shuffle::Deal(amount) => mod_div(card, amount, len),
        }
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

fn _run(shuffles: Vec<Shuffle>, size: CardInt) -> Vec<CardInt> {
    let mut cards: Vec<CardInt> = (0..size).collect();
    for shuffle in shuffles {
        cards = shuffle._apply(cards);
    }
    cards
}

fn apply(shuffles: &[Shuffle], size: CardInt, card: CardInt) -> Result<CardInt> {
    let mut card = card;
    for shuffle in shuffles {
        card = shuffle.apply_to(card, size);
    }
    Ok(card)
}

fn _run2(shuffles: &Vec<Shuffle>, size: CardInt, card: CardInt, times: usize) -> CardInt {
    let mut shuffles = shuffles.clone();
    let mut card = card;
    shuffles.reverse();
    for _ in 0..times {
        for shuffle in &shuffles {
            card = shuffle._apply_reverse_to(card, size);
        }
    }
    card
}

fn parse(input: &str) -> Result<Vec<Shuffle>> {
    input
        .split('\n')
        .map(|l| l.parse())
        .collect::<Result<Vec<Shuffle>>>()
}

fn part1(input: &str) -> Result<i64> {
    let shuffles = parse(input)?;
    apply(&shuffles, 10007, 2019)
}

fn part2(input: &str) -> Result<i64> {
    let shuffles = parse(input)?;
    let len = 119315717514047;
    let mut card = Variable::new(1, 0, len);
    for shuffle in &shuffles {
        card = shuffle.apply_to_variable(card, len);
    }
    /* dbg!(&card); */
    /* dbg!((mod_mul(2019, card.x, len) + card.c) % len); */
    /* dbg!(mod_div(8326 - card.c, card.x, len)); */
    /* dbg!(card.eval(2019)); */
    /* dbg!(card.invert().eval(8326)); */

    let mut pow = card;

    let times: i64 = 101741582076661;
    let digits = format!("{:b}", times).chars().rev().collect::<Vec<_>>();
    let mut total = Variable::new(1, 0, len);
    for digit in digits.into_iter() {
        if digit == '1' {
            total = total * pow;
        }
        pow = pow * pow;
    }
    Ok(total.invert().eval(2020))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_int() {
        assert_eq!(mod_inverse(5, 8), 5);
    }

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
    fn test_part1a() -> Result<()> {
        let shuffles = parse("deal into new stack")?;
        assert_eq!(apply(&shuffles, 10, 0)?, 9);
        assert_eq!(apply(&shuffles, 10, 1)?, 8);
        assert_eq!(apply(&shuffles, 10, 9)?, 0);
        Ok(())
    }

    #[test]
    fn test_part1b() -> Result<()> {
        let shuffles = parse("cut 3")?;
        assert_eq!(apply(&shuffles, 10, 0)?, 7);
        assert_eq!(apply(&shuffles, 10, 1)?, 8);
        assert_eq!(apply(&shuffles, 10, 9)?, 6);
        Ok(())
    }

    #[test]
    fn test_part1c() -> Result<()> {
        let shuffles = parse("cut -4")?;
        assert_eq!(apply(&shuffles, 10, 0)?, 6);
        assert_eq!(apply(&shuffles, 10, 1)?, 7);
        assert_eq!(apply(&shuffles, 10, 9)?, 5);
        Ok(())
    }

    #[test]
    fn test_part1d() -> Result<()> {
        let shuffles = parse("deal with increment 3")?;
        assert_eq!(apply(&shuffles, 10, 0)?, 0);
        assert_eq!(apply(&shuffles, 10, 1)?, 3);
        assert_eq!(apply(&shuffles, 10, 9)?, 7);
        Ok(())
    }

    #[test]
    fn test_part1e() -> Result<()> {
        let shuffles = parse(
            "deal with increment 3
deal into new stack",
        )?;
        // 0 7 4 1 8 5 2 9 6 3
        // 3 6 9 2 5 8 1 4 7 0
        assert_eq!(apply(&shuffles, 10, 0)?, 9);
        assert_eq!(apply(&shuffles, 10, 1)?, 6);
        assert_eq!(apply(&shuffles, 10, 9)?, 2);
        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<()> {
        let shuffles = parse("deal with increment 3")?;
        // 0 7 4 1 8 5 2 9 6 3
        assert_eq!(run2(&shuffles, 10, 0, 1), 0);
        assert_eq!(run2(&shuffles, 10, 1, 1), 7);
        assert_eq!(run2(&shuffles, 10, 9, 1), 3);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        let shuffles = parse("deal into new stack")?;
        // 9 8 7 6 5 4 3 2 1 0
        assert_eq!(run2(&shuffles, 10, 0, 1), 9);
        assert_eq!(run2(&shuffles, 10, 1, 1), 8);
        assert_eq!(run2(&shuffles, 10, 9, 1), 0);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }

    #[test]
    fn test_part2c() -> Result<()> {
        let shuffles = parse("cut 3")?;
        // 3 4 5 6 7 8 9 0 1 2
        assert_eq!(run2(&shuffles, 10, 0, 1), 3);
        assert_eq!(run2(&shuffles, 10, 1, 1), 4);
        assert_eq!(run2(&shuffles, 10, 9, 1), 2);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }

    #[test]
    fn test_part2d() -> Result<()> {
        let shuffles = parse("cut -4")?;
        // 6 7 8 9 0 1 2 3 4 5
        assert_eq!(run2(&shuffles, 10, 0, 1), 6);
        assert_eq!(run2(&shuffles, 10, 1, 1), 7);
        assert_eq!(run2(&shuffles, 10, 9, 1), 5);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }

    #[test]
    fn test_part2e() -> Result<()> {
        let shuffles = parse("cut 2\ndeal into new stack")?;
        // 2 3 4 5 6 7 8 9 0 1
        // 1 0 9 8 7 6 5 4 3 2
        assert_eq!(run2(&shuffles, 10, 0, 1), 1);
        assert_eq!(run2(&shuffles, 10, 1, 1), 0);
        assert_eq!(run2(&shuffles, 10, 9, 1), 2);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }

    #[test]
    fn test_ex1() -> Result<()> {
        let shuffles = parse(
            "deal with increment 7
deal into new stack
deal into new stack",
        )?;
        // 0 3 6 9 2 5 8 1 4 7
        assert_eq!(run2(&shuffles, 10, 0, 1), 0);
        assert_eq!(run2(&shuffles, 10, 1, 1), 3);
        assert_eq!(run2(&shuffles, 10, 9, 1), 7);
        // n -> n*amount % len
        Ok(())
    }

    #[test]
    fn test_incr9() -> Result<()> {
        let shuffles = parse("deal with increment 9")?;
        // 0 9 8 7 6 5 4 3 2 1
        assert_eq!(run2(&shuffles, 10, 0, 1), 0);
        assert_eq!(run2(&shuffles, 10, 1, 1), 9);
        assert_eq!(run2(&shuffles, 10, 9, 1), 1);
        Ok(())
    }

    #[test]
    fn test_exb1() -> Result<()> {
        let shuffles = parse(
            "cut 6
deal with increment 7
deal into new stack",
        )?;
        // 0 1 2 3 4 5 6 7 8 9
        // 6 7 8 9 0 1 2 3 4 5
        // 6 9 2 5 8 1 4 7 0 3
        // 3 0 7 4 1 8 5 2 9 6
        assert_eq!(apply(&shuffles, 10, 0)?, 1);
        assert_eq!(apply(&shuffles, 10, 1)?, 4);
        assert_eq!(apply(&shuffles, 10, 9)?, 8);
        Ok(())
    }

    #[test]
    fn test_exb2() -> Result<()> {
        let shuffles = parse(
            "cut 6
deal with increment 7
deal into new stack",
        )?;
        // 3 0 7 4 1 8 5 2 9 6
        assert_eq!(run2(&shuffles, 10, 0, 1), 3);
        assert_eq!(run2(&shuffles, 10, 1, 1), 0);
        assert_eq!(run2(&shuffles, 10, 9, 1), 6);
        Ok(())
    }

    #[test]
    fn test_ex3() -> Result<()> {
        let shuffles = parse(
            "deal with increment 7
deal with increment 9
cut -2",
        )?;
        //
        let results = (0..10)
            .map(|i| run2(&shuffles, 10, i, 1))
            .collect::<Vec<_>>();
        // let expected = "6 3 0 7 4 1 8 5 2 9".split(" ").
        assert_eq!(results, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
        // assert_eq!(run2(&shuffles, 10, 0), 6);
        // assert_eq!(run2(&shuffles, 10, 1), 3);
        // assert_eq!(run2(&shuffles, 10, 9), 9);
        Ok(())
    }

    #[test]
    fn test_cycle_length_cut() -> Result<()> {
        let shuffles = parse("cut 3")?;
        // 3 4 5 6 7 8 9 0 1 2
        // 6 7 8 9 0 1 2 3 4 5
        // 9 0 1 2 3 4 5 6 7 8
        // 2 3 4 5 6 7 8 9 0 1
        // 5 6 7 8 9 0 1 2 3 4
        assert_eq!(run2(&shuffles, 7, 0, 7), 0);
        // assert_eq!(run2(&shuffles, 10, 9), 7);
        Ok(())
    }
    // 101741582076661
    // 119315717514047

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(apply(&shuffles, 10, 0)?, 9);
        assert_eq!(apply(&shuffles, 10, 1)?, 8);
        assert_eq!(apply(&shuffles, 10, 9)?, 0);
        Ok(())
    }
}
