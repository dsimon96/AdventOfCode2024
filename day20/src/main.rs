use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead},
};

use anyhow::{bail, ensure, Context, Result};
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

type Coords = (usize, usize);

struct Map {
    height: usize,
    width: usize,
    is_wall: Vec<bool>,
    end: Coords,
}

fn parse_maze(inp: impl BufRead) -> Result<Map> {
    let lines = inp.lines().collect::<Result<Vec<_>, _>>()?;
    let height = lines.len();
    let (first, rest) = lines.split_first().context("Empty input")?;
    let width = first.len();
    ensure!(
        rest.iter().all(|s| s.len() == width),
        "Inconsistent line lengths"
    );

    let mut end = None;
    let mut is_wall = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.char_indices() {
            is_wall.push(match ch {
                '#' => true,
                '.' | 'S' => false,
                'E' => {
                    end = Some((r, c));
                    false
                }
                _ => bail!("Invalid char in input: {ch}"),
            })
        }
    }

    Ok(Map {
        height,
        width,
        end: end.context("Missing end")?,
        is_wall,
    })
}

fn adjacent(map: &Map, &(r, c): &Coords) -> Vec<Coords> {
    let mut res = Vec::new();
    if r > 0 {
        res.push((r - 1, c));
    }
    if r < map.height - 1 {
        res.push((r + 1, c));
    }
    if c > 0 {
        res.push((r, c - 1));
    }
    if c < map.width - 1 {
        res.push((r, c + 1));
    }

    res
}

const fn idx(map: &Map, &(r, c): &Coords) -> usize {
    r * map.width + c
}

fn calc_dists(map: &Map) -> Vec<Option<usize>> {
    let mut res = vec![None; map.height * map.width];

    res[idx(map, &map.end)] = Some(0);
    let mut q = VecDeque::from([(0, map.end)]);

    while let Some((dist, coords)) = q.pop_front() {
        for new_coords in adjacent(map, &coords) {
            let idx = idx(map, &new_coords);
            if map.is_wall[idx] || res[idx].is_some() {
                continue;
            }

            res[idx] = Some(dist + 1);
            q.push_back((dist + 1, new_coords));
        }
    }

    res
}

fn cheat_endpoints(map: &Map, start: &Coords, max_duration: usize) -> Vec<(usize, Coords)> {
    let mut res = Vec::new();
    let mut visited = HashSet::from([*start]);
    let mut q = VecDeque::from([(0, *start)]);

    while let Some((elapsed, coords)) = q.pop_front() {
        if elapsed == max_duration {
            continue;
        }

        for new_coords in adjacent(map, &coords) {
            if !visited.contains(&new_coords) {
                if !map.is_wall[idx(map, &new_coords)] {
                    res.push((elapsed + 1, new_coords));
                }
                visited.insert(new_coords);
                q.push_back((elapsed + 1, new_coords));
            }
        }
    }

    res
}

fn main() -> Result<()> {
    let args = Args::parse();
    let map = parse_maze(stdin().lock())?;
    let dists = calc_dists(&map);
    let max_duration = match args.part {
        Part::P1 => 2,
        Part::P2 => 20,
    };

    let mut count = 0;
    for r in 0..map.height {
        for c in 0..map.width {
            let start = (r, c);
            let start_idx = idx(&map, &start);
            if map.is_wall[start_idx] {
                continue;
            }
            let start_dist = dists[start_idx].context("Missing distance")?;

            for (elapsed, end) in cheat_endpoints(&map, &start, max_duration) {
                let end_dist = dists[idx(&map, &end)].context("Missing distance")?;

                if end_dist + elapsed + 100 <= start_dist {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");

    Ok(())
}
