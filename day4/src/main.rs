use std::io::{stdin, BufRead};

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

fn main() -> Result<()> {
    let args = Args::parse();

    let grid = parse_grid(stdin().lock())?;
    match args.part {
        Part::P1 => {
            const FORWARD: &str = "XMAS";
            const BACKWARD: &str = "SAMX";

            let mut total = 0;
            // horiz
            for r in 0..grid.len() {
                for c in 0..grid[0].len() - 3 {
                    let chars = (0..4usize).map(|dc| grid[r][c + dc]);
                    if chars.clone().eq(FORWARD.chars()) || chars.eq(BACKWARD.chars()) {
                        total += 1;
                    }
                }
            }

            // vert
            for r in 0..grid.len() - 3 {
                for c in 0..grid[0].len() {
                    let chars = (0..4usize).map(|dr| grid[r + dr][c]);
                    if chars.clone().eq(FORWARD.chars()) || chars.eq(BACKWARD.chars()) {
                        total += 1;
                    }
                }
            }

            // right diag
            for r in 0..grid.len() - 3 {
                for c in 0..grid[0].len() - 3 {
                    let chars = (0..4usize).map(|d| grid[r + d][c + d]);
                    if chars.clone().eq(FORWARD.chars()) || chars.eq(BACKWARD.chars()) {
                        total += 1;
                    }
                }
            }

            // left diag
            for r in 0..grid.len() - 3 {
                for c in 3..grid[0].len() {
                    let chars = (0..4usize).map(|d| grid[r + d][c - d]);
                    if chars.clone().eq(FORWARD.chars()) || chars.eq(BACKWARD.chars()) {
                        total += 1;
                    }
                }
            }

            println!("{total}");
        }
        Part::P2 => {
            const FORWARD: (char, char) = ('M', 'S');
            const BACKWARD: (char, char) = ('S', 'M');
            let mut total = 0;

            for r in 1..grid.len() - 1 {
                for c in 1..grid[0].len() - 1 {
                    if grid[r][c] != 'A' {
                        continue;
                    }

                    let rdiag = (grid[r - 1][c - 1], grid[r + 1][c + 1]);
                    let ldiag = (grid[r - 1][c + 1], grid[r + 1][c - 1]);

                    if (rdiag == FORWARD || rdiag == BACKWARD)
                        && (ldiag == FORWARD || ldiag == BACKWARD)
                    {
                        total += 1;
                    }
                }
            }

            println!("{total}");
        }
    }

    Ok(())
}

fn parse_grid(inp: impl BufRead) -> Result<Vec<Vec<char>>> {
    let mut res = Vec::new();

    for line in inp.lines() {
        res.push(line?.chars().collect());
    }

    Ok(res)
}
