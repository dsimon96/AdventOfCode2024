use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    ops::Index,
    rc::Rc,
};

use anyhow::{anyhow, ensure, Result};
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

struct Map {
    rows: usize,
    cols: usize,
    heights: Vec<usize>,
}

type Idx = (usize, usize);

impl Index<&Idx> for Map {
    type Output = usize;

    fn index(&self, &(r, c): &Idx) -> &Self::Output {
        &self.heights[r * self.cols + c]
    }
}

impl Map {
    fn trailheads(&self) -> impl Iterator<Item = Idx> + '_ {
        (0..self.rows)
            .flat_map(|r| (0..self.cols).map(move |c| (r, c)))
            .filter(|idx| self[idx] == 0)
    }

    fn adj_indices(&self, &(r, c): &Idx) -> impl Iterator<Item = Idx> {
        let mut indices = Vec::new();

        if r > 0 {
            indices.push((r - 1, c));
        }
        if r < self.rows - 1 {
            indices.push((r + 1, c));
        }
        if c > 0 {
            indices.push((r, c - 1));
        }
        if c < self.cols - 1 {
            indices.push((r, c + 1));
        }

        indices.into_iter()
    }
}

fn parse_line(s: &str) -> impl Iterator<Item = Result<usize>> + '_ {
    s.chars().map(|ch| {
        ch.to_digit(10).map_or_else(
            || Err(anyhow!("Non-numeric char in input")),
            |u| Ok(usize::try_from(u)?),
        )
    })
}

fn parse_map(inp: impl BufRead) -> Result<Map> {
    let mut heights = Vec::new();
    let mut rows = 0;
    let mut cols = None;
    for line in inp.lines() {
        let line = line?;
        let len = line.len();

        match cols {
            None => cols = Some(len),
            Some(cols) => ensure!(cols == len, "Inconsistent line lengths"),
        }

        rows += 1;
        for height in parse_line(&line) {
            heights.push(height?);
        }
    }
    ensure!(cols.is_some(), "Empty input");

    Ok(Map {
        rows,
        cols: cols.unwrap(),
        heights,
    })
}

type MemoTable = HashMap<Idx, Rc<HashMap<Idx, usize>>>;

fn calculate_reachable(memo: &mut MemoTable, map: &Map, idx: Idx) -> Rc<HashMap<Idx, usize>> {
    if let Some(res) = memo.get(&idx) {
        return res.clone();
    }

    let height = map[&idx];
    let reachable = if height == 9 {
        HashMap::from([(idx, 1)])
    } else {
        let mut reachable = HashMap::new();
        for other in map
            .adj_indices(&idx)
            .filter(|other| map[other] == height + 1)
        {
            for (&peak, &count) in calculate_reachable(memo, map, other).iter() {
                *reachable.entry(peak).or_default() += count;
            }
        }

        reachable
    };

    let rc = Rc::new(reachable);
    memo.insert(idx, rc.clone());

    rc
}

fn main() -> Result<()> {
    let args = Args::parse();
    let map = parse_map(stdin().lock())?;

    let mut memo = HashMap::new();
    let trailheads_reachability = map
        .trailheads()
        .map(|idx| calculate_reachable(&mut memo, &map, idx));
    let total_score: usize = match args.part {
        Part::P1 => trailheads_reachability.map(|m| m.len()).sum(),
        Part::P2 => trailheads_reachability
            .map(|m| m.values().sum::<usize>())
            .sum(),
    };
    println!("{total_score}");

    Ok(())
}
