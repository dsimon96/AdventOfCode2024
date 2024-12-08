use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead}};

use anyhow::{anyhow, Result};
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

type Position = (i64, i64);

struct Map {
    rows: usize,
    cols: usize,
    antennae: Vec<(char, Position)>
}

fn parse_map(r: impl BufRead) -> Result<Map> {
    let mut rows = 0;
    let mut cols = None;
    let mut antennae = Vec::new();
    for (r, line) in r.lines().enumerate() {
        rows += 1;
        let s = line?;
        if cols.is_none() { cols = Some(s.len()); }

        for (c, ch) in s.char_indices() {
            if ch == '.' { continue }
            else { antennae.push((ch, (r.try_into()?, c.try_into()?))) }
        }
    }
    let cols = cols.ok_or_else(|| anyhow!("Couldn't determine the width of the map. Is it empty?"))?;
    Ok(Map { rows, cols, antennae })
}

fn by_freq(map: &Map) -> HashMap<char, Vec<Position>> {
    let mut res: HashMap<char, Vec<Position>> = HashMap::new();

    for &(ch, pos) in map.antennae.iter() {
        res.entry(ch).or_default().push(pos);
    }

    res
}

fn antinodes_p1(rows: usize, cols: usize, x: &Position, y: &Position) -> Vec<Position> {
    let &(rx, cx) = x;
    let &(ry, cy) = y;

    let dr = ry - rx;
    let dc = cy - cx;

    let a1 = (rx - dr, cx - dc);
    let a2 = (ry + dr, cy + dc);

    let mut res = Vec::new();
    if is_in_bounds(rows, cols, &a1) { res.push(a1) }
    if is_in_bounds(rows, cols, &a2) { res.push(a2) }
    res
}

fn antinodes_p2(rows: usize, cols: usize, x: &Position, y: &Position) -> Vec<Position> {
    let &(rx, cx) = x;
    let &(ry, cy) = y;

    let dr = ry - rx;
    let dc = cy - cx;

    let mut res = Vec::new();

    let mut r = rx;
    let mut c = cx;
    while is_in_bounds(rows, cols, &(r, c)) {
        res.push((r, c));

        r -= dr;
        c -= dc;
    }

    r = ry;
    c = cy;
    while is_in_bounds(rows, cols, &(r, c)) {
        res.push((r, c));

        r += dr;
        c += dc;
    }

    res
}

fn is_in_bounds(rows: usize, cols: usize, pos: &Position) -> bool {
    let &(r, c) = pos;

    0 <= r && (r as usize) < rows && 0 <= c && (c as usize) < cols
}

fn main() -> Result<()> {
    let args = Args::parse();
    let map = parse_map(stdin().lock())?;
    let by_freq = by_freq(&map);

    let antinodes = match args.part {
        Part::P1 => antinodes_p1,
        Part::P2 => antinodes_p2,
    };

    let mut uniq = HashSet::new();
    for antennae in by_freq.values() {
        for (x, y) in antennae.iter().tuple_combinations() {
            uniq.extend(antinodes(map.rows, map.cols, x, y));
        }

    }

    println!("{}", uniq.len());

    Ok(())
}
