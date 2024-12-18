use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{stdin, BufRead},
};

use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum Part {
    P1 { n: usize },
    P2,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    part: Part,

    height: usize,
    width: usize,
}

type Coords = (usize, usize);

fn parse_coords(inp: impl BufRead) -> impl Iterator<Item = Result<Coords>> {
    inp.lines().map(|line| {
        let line = line?;
        let (xs, ys) = line
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid input"))?;
        Ok((xs.parse()?, ys.parse()?))
    })
}

fn surrounding((r, c): Coords, height: usize, width: usize) -> Vec<Coords> {
    let mut res = Vec::new();
    if r > 0 {
        res.push((r - 1, c));
    }
    if r < height {
        res.push((r + 1, c));
    }
    if c > 0 {
        res.push((r, c - 1));
    }
    if c < width {
        res.push((r, c + 1));
    }
    res
}

fn find_path(corrupted: &HashSet<Coords>, height: usize, width: usize) -> Option<Vec<Coords>> {
    let start = (0, 0);
    let end = (height, width);
    let mut prev = HashMap::new();
    prev.insert(start, None);
    let mut q = VecDeque::from([start]);
    'outer: loop {
        let coords = q.pop_front()?;

        for new_coord in surrounding(coords, height, width) {
            if corrupted.contains(&new_coord) || prev.contains_key(&new_coord) {
                continue;
            }

            prev.insert(new_coord, Some(coords));
            if new_coord == end {
                break 'outer;
            }
            q.push_back(new_coord);
        }
    }

    let mut cur = end;
    let mut path = Vec::from([end]);
    while let Some(Some(prev)) = prev.get(&cur) {
        path.push(*prev);
        cur = *prev;
    }
    Some(path)
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.part {
        Part::P1 { n } => {
            let corrupted = parse_coords(stdin().lock())
                .take(n)
                .collect::<Result<HashSet<_>>>()?;
            let Some(path) = find_path(&corrupted, args.height, args.width) else {
                bail!("Failed to find path");
            };
            println!("{}", path.len() - 1);
        }
        Part::P2 => {
            let mut corrupted = HashSet::new();
            let mut path: HashSet<(usize, usize)> =
                HashSet::from_iter(find_path(&corrupted, args.height, args.width).unwrap());
            for coord in parse_coords(stdin().lock()) {
                let coord = coord?;
                corrupted.insert(coord);
                if path.contains(&coord) {
                    let Some(new_path) = find_path(&corrupted, args.height, args.width) else {
                        println!("{},{}", coord.0, coord.1);
                        break;
                    };
                    path = HashSet::from_iter(new_path);
                }
            }
        }
    }

    Ok(())
}
