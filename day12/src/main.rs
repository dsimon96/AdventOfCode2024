use std::{
    collections::HashSet,
    hash::Hash,
    io::{stdin, BufRead},
    ops::Index,
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

type Plant = char;

struct Map {
    rows: usize,
    cols: usize,
    plants: Vec<Plant>,
}

impl Map {
    fn parse(inp: impl BufRead) -> Result<Self> {
        let mut rows = 0;
        let mut cols = None;
        let mut plants = Vec::new();

        for line in inp.lines() {
            let line = line?;
            if let Some(cols) = cols {
                ensure!(cols == line.len(), "Inconsistent line lengths");
            } else {
                cols = Some(line.len());
            }

            rows += 1;
            plants.extend(line.chars());
        }

        Ok(Self {
            rows,
            cols: cols.ok_or_else(|| anyhow!("Empty input"))?,
            plants,
        })
    }
}

type Coord = (usize, usize);

impl Index<Coord> for Map {
    type Output = Plant;

    fn index(&self, (r, c): Coord) -> &Self::Output {
        &self.plants[r * self.cols + c]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum EdgeOrientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    coord: Coord,
    orientation: EdgeOrientation,
}

fn measure_region(map: &Map, coords: Coord) -> (HashSet<Coord>, HashSet<Edge>) {
    let plant = map[coords];
    let mut to_visit = vec![coords];
    let mut seen = HashSet::from([coords]);
    let mut perimeter_edges = HashSet::new();

    while let Some((r, c)) = to_visit.pop() {
        for (dr, dc, edge) in [
            (
                -1,
                0,
                Edge {
                    coord: (r, c),
                    orientation: EdgeOrientation::Up,
                },
            ),
            (
                1,
                0,
                Edge {
                    coord: (r + 1, c),
                    orientation: EdgeOrientation::Down,
                },
            ),
            (
                0,
                -1,
                Edge {
                    coord: (r, c),
                    orientation: EdgeOrientation::Left,
                },
            ),
            (
                0,
                1,
                Edge {
                    coord: (r, c + 1),
                    orientation: EdgeOrientation::Right,
                },
            ),
        ] {
            let r = r.checked_add_signed(dr);
            let c = c.checked_add_signed(dc);

            match (r, c) {
                (Some(r), Some(c)) if r < map.rows && c < map.cols => {
                    let new_coords = (r, c);
                    if seen.contains(&new_coords) {
                        continue;
                    } else if map[new_coords] == plant {
                        seen.insert(new_coords);
                        to_visit.push(new_coords);
                    } else {
                        perimeter_edges.insert(edge);
                    }
                }
                _ => {
                    perimeter_edges.insert(edge);
                }
            }
        }
    }

    (seen, perimeter_edges)
}

fn measure_side(map: &Map, edges: &HashSet<Edge>, &edge: &Edge) -> Vec<Edge> {
    let mut side_edges = vec![edge];
    let directions = match edge.orientation {
        EdgeOrientation::Up | EdgeOrientation::Down => [(0, -1), (0, 1)],
        EdgeOrientation::Left | EdgeOrientation::Right => [(-1, 0), (1, 0)],
    };

    let Edge {
        coord: (r, c),
        orientation,
    } = edge;

    for (dr, dc) in directions {
        for dist in 1.. {
            let r = r.checked_add_signed(dr * dist);
            let c = c.checked_add_signed(dc * dist);

            match (r, c) {
                (Some(r), Some(c)) if r <= map.rows && c <= map.cols => {
                    let expected_edge = Edge {
                        coord: (r, c),
                        orientation,
                    };
                    if edges.contains(&expected_edge) {
                        side_edges.push(expected_edge);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    side_edges
}

fn count_sides(map: &Map, edges: &HashSet<Edge>) -> usize {
    let mut seen = HashSet::new();

    let mut count = 0;
    for edge in edges {
        if !seen.contains(edge) {
            seen.extend(measure_side(map, edges, edge));

            count += 1;
        }
    }

    count
}

fn main() -> Result<()> {
    let args = Args::parse();
    let map = Map::parse(stdin().lock())?;

    let mut seen = HashSet::new();

    let mut total = 0;
    for r in 0..map.rows {
        for c in 0..map.cols {
            if !seen.contains(&(r, c)) {
                let (contained, perimeter_edges) = measure_region(&map, (r, c));
                let price = match args.part {
                    Part::P1 => contained.len() * perimeter_edges.len(),
                    Part::P2 => contained.len() * count_sides(&map, &perimeter_edges),
                };
                total += price;

                seen.extend(contained.into_iter());
            }
        }
    }

    println!("{total}");
    Ok(())
}
