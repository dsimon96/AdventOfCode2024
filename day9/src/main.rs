use std::{collections::VecDeque, io::stdin};

use anyhow::Result;
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

type BlockId = usize;

struct Run {
    id: Option<BlockId>,
    len: usize,
}

fn parse_input(s: &str) -> VecDeque<Run> {
    s.char_indices()
        .map(|(i, ch)| {
            let id = if i % 2 == 0 { Some(i / 2) } else { None };
            let len = ch.to_digit(10).expect("Non-numeric input char");

            Run {
                id,
                len: len.try_into().unwrap(),
            }
        })
        .collect()
}

struct P1Iterator {
    cur_block_id: Option<BlockId>,
    cur_len: usize,
    offset: usize,
    remaining_runs: VecDeque<Run>,
}

impl P1Iterator {
    fn new(runs: VecDeque<Run>) -> P1Iterator {
        P1Iterator {
            cur_block_id: None,
            cur_len: 0,
            offset: 0,
            remaining_runs: runs,
        }
    }
}

impl Iterator for P1Iterator {
    type Item = Option<BlockId>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_block_id.is_none() || self.offset >= self.cur_len {
            // starting a new block
            self.offset = 0;

            let Some(first) = self.remaining_runs.pop_front() else {
                return None;
            };
            if first.id.is_some() {
                self.cur_block_id = first.id;
                self.cur_len = first.len;
            } else {
                // fill empty blocks from the rightmost non-empty run
                let fill_from = loop {
                    let Some(rightmost) = self.remaining_runs.pop_back() else {
                        return None;
                    };
                    if rightmost.id.is_some() {
                        break rightmost;
                    }
                };

                self.cur_block_id = fill_from.id;
                self.cur_len = first.len.min(fill_from.len);

                if first.len > fill_from.len {
                    // there are still empty blocks to be filled
                    self.remaining_runs.push_front(Run {
                        id: None,
                        len: first.len - fill_from.len,
                    });
                } else if first.len < fill_from.len {
                    // there are leftover blocks from the fill-from run
                    self.remaining_runs.push_back(Run {
                        id: fill_from.id,
                        len: fill_from.len - first.len,
                    });
                }
            }
        }

        self.offset += 1;
        Some(Some(self.cur_block_id.unwrap()))
    }
}

fn checksum(it: impl Iterator<Item = Option<BlockId>>) -> usize {
    it.enumerate()
        .map(|(i, id)| i * id.unwrap_or_default())
        .sum()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let runs = parse_input(&stdin().lines().next().expect("Unexpected empty input")?);

    match args.part {
        Part::P1 => println!("{}", checksum(P1Iterator::new(runs))),
        Part::P2 => todo!(),
    }

    Ok(())
}
