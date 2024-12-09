use anyhow::Result;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Failed to parse input")]
    ParseError,
}

fn parse_lists(inp: impl BufRead) -> Result<(Vec<u64>, Vec<u64>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in inp.lines() {
        let s = line?;
        let (ls, rs) = s.split_once("   ").ok_or(Error::ParseError)?;

        left.push(ls.parse()?);
        right.push(rs.parse()?);
    }

    Ok((left, right))
}

fn total_distance(mut l: Vec<u64>, mut r: Vec<u64>) -> u64 {
    l.sort_unstable();
    r.sort_unstable();

    l.iter().zip(r.iter()).map(|(&l, &r)| l.abs_diff(r)).sum()
}

fn freq_map(v: &[u64]) -> HashMap<u64, u64> {
    let mut res = HashMap::new();
    for &elem in v {
        *res.entry(elem).or_default() += 1;
    }

    res
}

fn similarity_score(l: &[u64], r: &[u64]) -> u64 {
    let right_freqs = freq_map(r);

    l.iter()
        .map(|&v| v * right_freqs.get(&v).copied().unwrap_or(0))
        .sum()
}

fn main() -> Result<()> {
    let (l, r) = parse_lists(stdin().lock())?;

    println!("{}", total_distance(l.clone(), r.clone()));
    println!("{}", similarity_score(&l, &r));

    Ok(())
}
