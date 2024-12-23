use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rand::{seq::IteratorRandom, thread_rng};

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

fn connection(s: &str) -> Result<(String, String)> {
    s.split_once('-')
        .context("Parsing line")
        .map(|(a, b)| (a.to_string(), b.to_string()))
}

fn connections(s: impl BufRead) -> impl Iterator<Item = Result<(String, String)>> {
    s.lines()
        .map(|line| connection(&line.context("Reading input")?))
}

type Graph = HashMap<String, HashSet<String>>;

fn bron_kerbosch(
    graph: &Graph,
    r: &HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    cb: &mut impl FnMut(&HashSet<String>),
) {
    if p.is_empty() && x.is_empty() {
        cb(r);
    } else {
        let mut rng = thread_rng();
        let pivot = p.union(&x).choose(&mut rng).unwrap();
        let vertices: Vec<String> = p.difference(&graph[pivot]).cloned().collect();
        for v in vertices {
            let vn = &graph[&v];
            bron_kerbosch(
                graph,
                &r.union(&HashSet::from([v.clone()])).cloned().collect(),
                p.intersection(vn).cloned().collect(),
                x.intersection(vn).cloned().collect(),
                cb,
            );
            p.remove(&v);
            x.insert(v.clone());
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let connections = connections(stdin().lock());
    let mut edges: Graph = HashMap::new();
    match args.part {
        Part::P1 => {
            let mut count = 0;
            for c in connections {
                let (a, b) = c?;

                edges.entry(a.clone()).or_default().insert(b.clone());
                edges.entry(b.clone()).or_default().insert(a.clone());

                let a_neighbors = edges.get(&a).unwrap();
                let b_neighbors = edges.get(&b).unwrap();

                for c in a_neighbors.intersection(b_neighbors) {
                    if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                        count += 1;
                    }
                }
            }
            println!("{count}");
        }
        Part::P2 => {
            for c in connections {
                let (a, b) = c?;

                edges.entry(a.clone()).or_default().insert(b.clone());
                edges.entry(b.clone()).or_default().insert(a.clone());
            }
            let all_vertices = edges.keys().cloned().collect::<HashSet<_>>();

            let mut best = HashSet::new();
            bron_kerbosch(
                &edges,
                &HashSet::new(),
                all_vertices,
                HashSet::new(),
                &mut |s| {
                    if s.len() > best.len() {
                        best.clone_from(s);
                    }
                },
            );

            let mut best = best.into_iter().collect::<Vec<_>>();
            best.sort();
            println!("{}", best.join(","));
        }
    }

    Ok(())
}
