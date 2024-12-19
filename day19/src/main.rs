use std::{
    collections::HashMap,
    io::{stdin, Error},
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

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

fn parse_towels(inp: &mut impl Iterator<Item = Result<String, Error>>) -> Result<Vec<String>> {
    let line = inp.next().context("Unexpected end of input")??;
    Ok(line.split(", ").map(str::to_string).collect())
}

fn patterns(
    inp: &mut impl Iterator<Item = Result<String, Error>>,
) -> impl Iterator<Item = Result<String>> + '_ {
    inp.map(|line| Ok(line?))
}

fn count_ways_recursive<'a>(
    memo: &mut HashMap<String, usize>,
    towels: &Vec<String>,
    pattern: &'a str,
) -> usize {
    if pattern.is_empty() {
        return 1;
    } else if let Some(&res) = memo.get(pattern) {
        return res;
    }

    let mut sum = 0;
    for towel in towels {
        if let Some(count) = pattern
            .strip_prefix(towel)
            .map(|rem| count_ways_recursive(memo, towels, rem))
        {
            sum += count;
        }
    }

    memo.insert(pattern.to_string(), sum);
    sum
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut inp = stdin().lines();
    let towels = parse_towels(&mut inp)?;
    inp.next().context("Unexpected end of input")??;

    let mut count = 0;
    let mut total_ways = 0;
    let mut memo = HashMap::new();
    for pat in patterns(&mut inp) {
        let pat = pat?;
        let ways = count_ways_recursive(&mut memo, &towels, &pat);
        if ways > 0 {
            count += 1;
        }
        total_ways += ways;
    }

    match args.part {
        Part::P1 => println!("{count}"),
        Part::P2 => println!("{total_ways}"),
    }

    Ok(())
}
