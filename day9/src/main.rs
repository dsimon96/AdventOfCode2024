use std::{collections::VecDeque, io::stdin, iter};

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

#[derive(Debug)]
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
    remaining_runs: VecDeque<Run>,
}

fn p1_compact(runs: VecDeque<Run>) -> P1Iterator {
    P1Iterator {
        remaining_runs: runs,
    }
}

impl Iterator for P1Iterator {
    type Item = Run;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(first) = self.remaining_runs.pop_front() else {
            return None;
        };
        if first.id.is_some() {
            Some(first)
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

            Some(Run {
                id: fill_from.id,
                len: first.len.min(fill_from.len),
            })
        }
    }
}

fn p2_compact(mut runs: VecDeque<Run>) -> impl Iterator<Item = Run> {
    let mut compacted = Vec::new();

    while let Some(run) = runs.pop_front() {
        if run.id.is_some() {
            compacted.push(run)
        } else {
            // find the rightmost file which can be moved, if one exists
            let mut tmp = Vec::new();
            let to_move = loop {
                let Some(rightmost) = runs.pop_back() else {
                    break None;
                };
                if rightmost.id.is_some() && rightmost.len <= run.len {
                    break Some(rightmost);
                }
                if !tmp.is_empty() || rightmost.id.is_some() {
                    tmp.push(rightmost);
                }
            };

            if let Some(moved) = to_move {
                let moved_len = moved.len;
                compacted.push(moved);
                if moved_len < run.len {
                    runs.push_front(Run {
                        id: None,
                        len: run.len - moved_len,
                    })
                }
                // leave empty space where the run was moved from
                runs.push_back(Run {
                    id: None,
                    len: moved_len,
                });
            } else {
                compacted.push(run);
            }
            // replace the elements removed from the end of the queue
            runs.extend(tmp.into_iter().rev());
        }
    }

    compacted.into_iter()
}

fn checksum(it: impl Iterator<Item = Run>) -> usize {
    it.flat_map(|run| iter::repeat_n(run.id, run.len))
        .enumerate()
        .map(|(i, id)| i * id.unwrap_or_default())
        .sum()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let runs = parse_input(&stdin().lines().next().expect("Unexpected empty input")?);

    match args.part {
        Part::P1 => println!("{}", checksum(p1_compact(runs))),
        Part::P2 => println!("{}", checksum(p2_compact(runs))),
    }

    Ok(())
}
