use anyhow::Result;
use aoc24::{get_input, Day};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

struct DayThree {
    input: String,
}

impl DayThree {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Day for DayThree {
    type Res = Result<u32>;
    fn part_one(&self) -> Self::Res {
        let (_input, idents) = parse(&self.input).unwrap();
        let total = idents
            .iter()
            .filter_map(|ident| match ident {
                Ident::Mul { lhs, rhs } => Some(*lhs * *rhs),
                Ident::Do => None,
                Ident::Dont => None,
            })
            .sum::<u32>();
        Ok(total)
    }

    fn part_two(&self) -> Self::Res {
        let (_input, idents) = parse(&self.input).unwrap();

        Ok(idents
            .iter()
            .fold((true, 0), |(status, value), ident| match ident {
                Ident::Mul { lhs, rhs } => {
                    if status {
                        (true, value + *lhs * *rhs)
                    } else {
                        (status, value)
                    }
                }
                Ident::Do => (true, value),
                Ident::Dont => (false, value),
            })
            .1)
    }

    fn from_input() -> Result<Self, std::io::Error> {
        let input = get_input(3)?;
        Ok(DayThree { input })
    }
}

fn mul(input: &str) -> IResult<&str, Ident> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((
        input,
        Ident::Mul {
            lhs: pair.0,
            rhs: pair.1,
        },
    ))
}

fn identifier(input: &str) -> IResult<&str, Ident> {
    alt((
        value(Ident::Do, tag("do()")),
        value(Ident::Dont, tag("don't()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Ident>> {
    many1(many_till(anychar, identifier).map(|(_, ident)| ident))(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Ident {
    Mul { lhs: u32, rhs: u32 },
    Do,
    Dont,
}

fn main() -> Result<(), std::io::Error> {
    let day_two = DayThree::from_input()?;
    aoc24::run(day_two)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_p1() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let day = DayThree::new(input.to_string());
        let total = day.part_one()?;

        assert_eq!(total, 161);
        Ok(())
    }
    #[test]
    fn test_parse_p2() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let day = DayThree::new(input.to_string());
        let total = day.part_two()?;

        assert_eq!(total, 48);
        Ok(())
    }
}
