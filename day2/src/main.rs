use anyhow::Result;
use itertools::Itertools;
use std::{
    io::{stdin, BufRead},
    num::ParseIntError,
};

type Report = Vec<u64>;

fn parse_report(line: &str) -> Result<Vec<u64>, ParseIntError> {
    line.split_whitespace().map(str::parse).collect()
}

fn parse_reports(inp: impl BufRead) -> Result<Vec<Report>> {
    let mut results = Vec::new();

    for line in inp.lines() {
        let line = line?;

        results.push(parse_report(&line)?);
    }

    Ok(results)
}

fn is_safe(report: &Report) -> bool {
    let mut orders = Vec::new();
    for (&prev, &cur) in report.iter().tuple_windows() {
        orders.push(cur.cmp(&prev));
        let mag = cur.abs_diff(prev);
        if !(1..=3).contains(&mag) {
            return false;
        }
    }

    orders.iter().all_equal()
}

fn is_safe_with_removal(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut with_removed = report.clone();
        with_removed.remove(i);

        if is_safe(&with_removed) {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    let reports = parse_reports(stdin().lock())?;

    println!("{}", reports.iter().filter(|r| is_safe(r)).count());
    println!(
        "{}",
        reports.iter().filter(|r| is_safe_with_removal(r)).count()
    );

    Ok(())
}
