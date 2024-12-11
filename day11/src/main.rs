use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use anyhow::{anyhow, Result};
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

fn parse_inp(inp: impl BufRead) -> Result<Vec<u64>> {
    let line = inp.lines().next().ok_or_else(|| anyhow!("Empty input"))??;
    line.split_whitespace()
        .map(|w| Ok(w.parse::<u64>()?))
        .collect::<Result<Vec<_>, _>>()
}

const fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap() + 1
}

const fn split_digits(n: u64, at: u32) -> [u64; 2] {
    let divisor = 10_u64.pow(at);
    [n / divisor, n % divisor]
}

fn transform(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let digits = num_digits(stone);
    if digits % 2 == 0 {
        split_digits(stone, digits / 2).into()
    } else {
        vec![stone * 2024]
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stone_counts: HashMap<u64, usize> = HashMap::new();
    for stone in parse_inp(stdin().lock())? {
        *stone_counts.entry(stone).or_default() += 1;
    }

    let num_iters = match args.part {
        Part::P1 => 25,
        Part::P2 => 75,
    };

    for _ in 0..num_iters {
        let mut new_counts = HashMap::new();
        for (stone, count) in stone_counts {
            for new_stone in transform(stone) {
                *new_counts.entry(new_stone).or_default() += count;
            }
        }

        stone_counts = new_counts;
    }

    println!("{}", stone_counts.into_values().sum::<usize>());

    Ok(())
}
