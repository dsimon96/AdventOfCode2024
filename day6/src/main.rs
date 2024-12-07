use std::{collections::HashSet, io::{stdin, BufRead}};

use anyhow::{anyhow, bail, Result};
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

type Position = (usize, usize);

struct Input {
    rows: usize,
    cols: usize,
    obstructions: Vec<Position>,
    init_pos: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

impl Direction {
    fn rotate (self: Self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH,
            Direction::WEST => Direction::NORTH,
        }
    }
}

fn parse_input(r: impl BufRead) -> Result<Input> {
    let mut obstructions = Vec::new();
    let mut init_pos = None;
    let mut rows = 0;
    let mut cols = 0;

    for (r, line) in r.lines().enumerate() {
        let line = line?;
        rows = rows.max(r+1);
        for (c, chr) in line.char_indices() {
            cols = cols.max(c+1);
            match chr {
                '.' => continue,
                '^' => init_pos = Some((r, c)),
                '#' => obstructions.push((r, c)),
                _ => bail!("Unexpected character '{}' in input", chr),
            }

        }
    }

    let init_pos = init_pos.ok_or(anyhow!("Could not find initial position"))?;

    Ok(Input { rows, cols, obstructions, init_pos })
}

fn next_pos(rows: usize, cols: usize, pos: Position, dir: Direction) -> Option<Position> {
    let (r, c) = pos;

    match dir {
        Direction::NORTH if r > 0  => Some ((r - 1, c)),
        Direction::SOUTH if r < rows - 1 => Some ((r + 1, c)),
        Direction::WEST if c > 0 => Some ((r, c - 1)),
        Direction::EAST if c < cols - 1 => Some ((r, c + 1)),
        _ => None
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let inp = parse_input(stdin().lock())?;
    let obstructions: HashSet<Position> = inp.obstructions.into_iter().collect();

    let mut pos = inp.init_pos;
    let mut dir = Direction::NORTH;
    match args.part {
        Part::P1 => {
            let mut covered= HashSet::new();
            covered.insert(pos);
            while let Some(next) = next_pos(inp.rows, inp.cols, pos, dir) {
                if obstructions.contains(&next) {
                    dir = dir.rotate();
                } else {
                    pos = next;
                    covered.insert(pos);
                }
            }
            println!("{}", covered.len());
        }
        Part::P2 => {
            let mut covered= HashSet::new();
            let mut possible_obstructions = HashSet::new();
            covered.insert(pos);
            let mut seen_collisions = HashSet::new();
            while let Some(next) = next_pos(inp.rows, inp.cols, pos, dir) {
                if obstructions.contains(&next) {
                    seen_collisions.insert((next, dir));
                    dir = dir.rotate();
                } else {
                    if !covered.contains(&next) {
                        // if we placed an obstruction here, would we loop?
                        let mut sub_pos = pos;
                        let mut sub_dir = dir.rotate();
                        let mut sub_obstructions = obstructions.clone();
                        sub_obstructions.insert(next);
                        let mut sub_seen_collisions = seen_collisions.clone();
                        sub_seen_collisions.insert((next, dir));

                        let mut is_loop = false;
                        while let Some(sub_next) = next_pos(inp.rows, inp.cols, sub_pos, sub_dir) {
                            if sub_obstructions.contains(&sub_next) {
                                if !sub_seen_collisions.insert((sub_next, sub_dir)) {
                                    is_loop = true;
                                    break;
                                }
                                sub_dir = sub_dir.rotate();
                            } else {
                                sub_pos = sub_next;
                            }
                        }

                        if is_loop {
                            possible_obstructions.insert(next);
                        }
                    }

                    pos = next;
                    covered.insert(pos);
                }
            }

            println!("{}", possible_obstructions.len());
        },
    }

    Ok(())
}
