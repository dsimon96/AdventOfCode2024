use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::map_res,
    error::Error,
    multi::{many0, many_till},
    sequence::separated_pair,
    Err, IResult,
};
use std::io::{read_to_string, stdin};

enum Operation {
    Do,
    Dont,
    Mul(u64, u64),
}

fn opdo(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("do()")(input)?;

    Ok((input, Operation::Do))
}

fn opdont(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("don't()")(input)?;

    Ok((input, Operation::Dont))
}

fn num(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn opmul(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (x, y)) = separated_pair(num, char(','), num)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Operation::Mul(x, y)))
}

fn operation(s: &str) -> IResult<&str, Operation> {
    let (s, (_, op)) = many_till(anychar, alt((opdo, opdont, opmul)))(s)?;

    Ok((s, op))
}

fn operations(s: &str) -> IResult<&str, Vec<Operation>> {
    many0(operation)(s)
}

fn interpret(ops: &Vec<Operation>) -> u64 {
    let mut enabled = true;
    let mut sum = 0;
    for op in ops {
        match op {
            Operation::Do => {
                enabled = true;
            }
            Operation::Dont => {
                enabled = false;
            }
            Operation::Mul(x, y) => {
                if enabled {
                    sum += x * y;
                }
            }
        }
    }

    sum
}

fn main() -> Result<()> {
    let s = read_to_string(stdin())?;
    let (_, operations) = operations(&s).map_err(Err::<Error<&str>>::to_owned)?;

    println!(
        "{}",
        operations
            .iter()
            .filter_map(|op| {
                let Operation::Mul(x, y) = op else {
                    return None;
                };

                Some(x * y)
            })
            .sum::<u64>()
    );

    println!("{}", interpret(&operations));

    Ok(())
}
