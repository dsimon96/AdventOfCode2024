use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    io::{stdin, BufRead},
};

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

type Coords = (usize, usize);

struct Map {
    start: Coords,
    end: Coords,
    walls: HashSet<Coords>,
}

fn parse_map(inp: impl BufRead) -> Result<Map> {
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();
    for (r, line) in inp.lines().enumerate() {
        let line = line?;
        for (c, ch) in line.char_indices() {
            match ch {
                '.' => {
                    continue;
                }
                '#' => {
                    walls.insert((r, c));
                }
                'S' => {
                    if let Some(other) = start {
                        bail!("Found multiple starts at {:?}, {other:?}", (r, c));
                    }
                    start = Some((r, c));
                }
                'E' => {
                    if let Some(other) = start {
                        bail!("Found multiple ends at {:?}, {other:?}", (r, c));
                    }
                    end = Some((r, c));
                }
                _ => bail!("Unrecognized character in input: {ch}"),
            }
        }
    }

    Ok(Map {
        start: start.ok_or_else(|| anyhow!("Missing start"))?,
        end: end.ok_or_else(|| anyhow!("Missing end"))?,
        walls,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn rot_ccw(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    const fn rot_cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
}

const fn go_forward((r, c): Coords, dir: Direction) -> Coords {
    match dir {
        Direction::North => (r - 1, c),
        Direction::South => (r + 1, c),
        Direction::East => (r, c + 1),
        Direction::West => (r, c - 1),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let map = parse_map(stdin().lock())?;

    let state = (map.start, Direction::East);
    let mut to_visit = BinaryHeap::new();
    let mut min_dist = HashMap::new();
    to_visit.push(Reverse((0, state)));
    min_dist.insert(state, (0, HashSet::from([map.start])));

    let best_score = loop {
        let Some(Reverse((score, (coords, dir)))) = to_visit.pop() else {
            bail!("No path");
        };

        if coords == map.end {
            break score;
        }
        let (best_seen, best_path) = min_dist.get(&(coords, dir)).unwrap();
        if *best_seen < score {
            // already checked a better path to this state
            continue;
        }
        let path = best_path.clone();

        let mut new_states = Vec::new();
        let new_coords = go_forward(coords, dir);
        if !map.walls.contains(&new_coords) {
            new_states.push((score + 1, (new_coords, dir)));
        }
        for new_dir in [dir.rot_ccw(), dir.rot_cw()] {
            new_states.push((score + 1000, (coords, new_dir)));
        }

        for (score, new_state) in new_states {
            let mut e = min_dist.entry(new_state);
            if let Entry::Occupied(o) = &mut e {
                let (best_seen, best_path) = o.get_mut();
                match (*best_seen).cmp(&score) {
                    Ordering::Less => {
                        continue;
                    }
                    Ordering::Equal => {
                        // another equally good path to the same state
                        best_path.extend(path.iter());
                        continue;
                    }
                    Ordering::Greater => {}
                }
            }

            // new best path to this state
            let mut new_path = path.clone();
            new_path.insert(new_state.0);
            e.insert_entry((score, new_path));
            to_visit.push(Reverse((score, new_state)));
        }
    };

    match args.part {
        Part::P1 => {
            println!("{best_score}");
        }
        Part::P2 => {
            // There could be best paths that approach the end from different directions
            let mut on_path: HashSet<Coords> = HashSet::new();
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                if let Some((score, best_path)) = min_dist.get(&(map.end, direction)) {
                    if *score == best_score {
                        on_path.extend(best_path.iter());
                    }
                }
            }

            println!("{}", on_path.len());
        }
    }

    Ok(())
}
