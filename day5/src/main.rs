use std::{
    collections::{HashMap, HashSet},
    io::{read_to_string, stdin, BufRead},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use nom::{
    character::complete::{char, digit1, line_ending, multispace1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Err, IResult,
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

type Rule = (u64, u64);
type Update = Vec<u64>;

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

fn num(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    separated_pair(num, char('|'), num)(input)
}

fn update(input: &str) -> IResult<&str, Update> {
    separated_list1(char(','), num)(input)
}

fn input(input: &str) -> IResult<&str, Input> {
    let (input, rules) = separated_list1(line_ending, rule)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, updates) = separated_list1(line_ending, update)(input)?;

    Ok((input, Input { rules, updates }))
}

fn parse_input(r: impl BufRead) -> Result<Input> {
    let s = read_to_string(r)?;
    let (_, input) = input(&s).map_err(Err::<Error<&str>>::to_owned)?;

    Ok(input)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = parse_input(stdin().lock())?;

    let mut deps: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (x, y) in input.rules {
        deps.entry(y).or_default().insert(x);
    }

    let mut correct_sum = 0;
    let mut incorrect_sum = 0;
    for update in input.updates {
        let mut disallowed: HashMap<u64, HashSet<u64>> = HashMap::new();
        let mut correct_order = true;
        let mut corrected = Vec::new();
        for &page in &update {
            if let Some(others) = disallowed.get(&page) {
                correct_order = false;
                let idx = corrected.iter().position(|p| others.contains(p)).unwrap();
                corrected.insert(idx, page);
            } else {
                corrected.push(page);
            }
            if let Some(expected) = deps.get(&page) {
                for &other in expected {
                    disallowed.entry(other).or_default().insert(page);
                }
            }
        }

        if correct_order {
            correct_sum += update[update.len() / 2];
        } else {
            incorrect_sum += corrected[corrected.len() / 2];
        }
    }

    match args.part {
        Part::P1 => {
            println!("{correct_sum}");
        }
        Part::P2 => {
            println!("{incorrect_sum}");
        }
    }

    Ok(())
}
