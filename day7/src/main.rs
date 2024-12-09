use std::io::stdin;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    Err, Finish, IResult,
};

#[derive(Debug, Subcommand)]
enum Part {
    P1,
    P2,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    part: Part,
}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

fn number(s: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(s)
}

fn equation(s: &str) -> IResult<&str, Equation> {
    let (s, test_value) = number(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, operands) = separated_list1(space1, number)(s)?;

    Ok((
        s,
        Equation {
            test_value,
            operands,
        },
    ))
}

fn try_peel_digits(target: u64, operand: u64) -> Option<u64> {
    let target_string = target.to_string();
    let operand_string = operand.to_string();
    target_string.strip_suffix(&operand_string).map(|s| {
        if s.is_empty() {
            0
        } else {
            s.parse().unwrap()
        }
    })
}

fn can_make(target: u64, operands: &[u64], use_concatenation: bool) -> bool {
    match operands.split_last() {
        Some((&last, remaining)) => {
            if last > target {
                return false;
            } else if target % last == 0 && can_make(target / last, remaining, use_concatenation) {
                return true;
            } else if use_concatenation {
                if let Some(new_target) = try_peel_digits(target, last) {
                    if can_make(new_target, remaining, use_concatenation) {
                        return true;
                    }
                }
            }
            can_make(target - last, remaining, use_concatenation)
        }
        None => target == 0,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let use_concatenation = matches!(args.part, Part::P2);
    let mut sum = 0;
    for s in stdin().lines() {
        let (_, eq) = equation(&s?)
            .map_err(Err::<Error<&str>>::to_owned)
            .finish()?;
        if can_make(eq.test_value, &eq.operands, use_concatenation) {
            sum += eq.test_value;
        }
    }

    println!("{sum}");

    Ok(())
}
