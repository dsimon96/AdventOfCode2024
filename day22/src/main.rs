use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    ops::BitXor,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use itertools::Itertools;

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

struct SecretGenerator(usize);

const fn prune(i: usize) -> usize {
    i % 16_777_216
}

impl Iterator for SecretGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.0;

        self.0 = prune(self.0.bitxor(self.0 * 64));
        self.0 = prune(self.0.bitxor(self.0 / 32));
        self.0 = prune(self.0.bitxor(self.0 * 2048));

        Some(prev)
    }
}

struct Buyer {
    secret: SecretGenerator,
}

impl Iterator for Buyer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.secret.next().unwrap() % 10)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let buyers: Vec<Buyer> = stdin()
        .lines()
        .map(|line| -> Result<Buyer> {
            let line = line?;
            Ok(Buyer {
                secret: SecretGenerator(line.as_str().parse::<usize>()?),
            })
        })
        .try_collect()?;

    match args.part {
        Part::P1 => println!(
            "{}",
            buyers
                .into_iter()
                .map(|mut buyer| buyer.secret.nth(2000).unwrap())
                .sum::<usize>()
        ),
        Part::P2 => {
            let mut seq_totals: HashMap<_, usize> = HashMap::new();
            for buyer in buyers {
                let mut seen = HashSet::new();
                for ((_, s1), (_, s2), (_, s3), (cur, s4)) in buyer
                    .take(2000)
                    .tuple_windows()
                    .map(|(prev, cur)| {
                        (
                            cur,
                            i8::try_from(cur).unwrap() - i8::try_from(prev).unwrap(),
                        )
                    })
                    .tuple_windows()
                {
                    let seq = [s1, s2, s3, s4];
                    if seen.insert(seq) {
                        *seq_totals.entry(seq).or_default() += cur;
                    }
                }
            }

            println!("{}", seq_totals.into_values().max().unwrap());
        }
    }

    Ok(())
}
