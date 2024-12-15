use std::{
    collections::{HashMap, HashSet},
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

type Coord = (usize, usize);

struct Map {
    walls: HashSet<Coord>,
    boxes: HashSet<Coord>,
    boxes_by_loc: HashMap<Coord, Coord>,
    robot: Coord,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(inp: impl BufRead, part: &Part) -> Result<(Map, Vec<Direction>)> {
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut boxes_by_loc = HashMap::new();
    let mut robot = None;
    let mut lines = inp.lines().enumerate();
    for (r, line) in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (c, ch) in line.char_indices() {
            match (ch, part) {
                ('.', _) => {
                    continue;
                }
                ('#', Part::P1) => {
                    walls.insert((r, c));
                }
                ('#', Part::P2) => {
                    walls.insert((r, 2 * c));
                    walls.insert((r, 2 * c + 1));
                }
                ('O', Part::P1) => {
                    let coords = (r, c);
                    boxes.insert(coords);
                    boxes_by_loc.insert(coords, coords);
                }
                ('O', Part::P2) => {
                    let coords = (r, 2 * c);
                    boxes.insert(coords);
                    boxes_by_loc.insert(coords, coords);
                    boxes_by_loc.insert((r, 2 * c + 1), coords);
                }
                ('@', Part::P1) => {
                    robot = Some((r, c));
                }
                ('@', Part::P2) => {
                    robot = Some((r, 2 * c));
                }
                _ => bail!("Unrecognized character in input: {ch}"),
            };
        }
    }

    let robot = robot.ok_or_else(|| anyhow!("Missing robot in input"))?;
    let map = Map {
        walls,
        boxes,
        boxes_by_loc,
        robot,
    };

    let mut moves = Vec::new();
    for (_, line) in lines {
        let line = line?;
        for ch in line.chars() {
            let mv = match ch {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => bail!("Unrecognized character in input: {ch}"),
            };

            moves.push(mv);
        }
    }

    Ok((map, moves))
}

const fn dest(&(r, c): &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => (r - 1, c),
        Direction::Down => (r + 1, c),
        Direction::Left => (r, c - 1),
        Direction::Right => (r, c + 1),
    }
}

fn boxes_to_move(map: &Map, coord: Coord, dir: &Direction, part: &Part) -> Option<HashSet<Coord>> {
    if map.walls.contains(&coord) {
        None
    } else if let Some(&(r, c)) = map.boxes_by_loc.get(&coord) {
        let dests = match (dir, part) {
            (_, Part::P1) | (Direction::Left, Part::P2) => vec![dest(&(r, c), dir)],
            (Direction::Right, Part::P2) => vec![dest(&(r, c + 1), dir)],
            (Direction::Up | Direction::Down, Part::P2) => {
                vec![dest(&(r, c), dir), dest(&(r, c + 1), dir)]
            }
        };

        let mut to_move = HashSet::from([(r, c)]);
        for dest in dests {
            if let Some(others) = boxes_to_move(map, dest, dir, part) {
                to_move.extend(others.into_iter());
            } else {
                return None;
            }
        }

        Some(to_move)
    } else {
        Some(HashSet::new())
    }
}

fn move_all_boxes(map: &mut Map, to_move: HashSet<Coord>, dir: &Direction, part: &Part) {
    for &(r, c) in &to_move {
        map.boxes.remove(&(r, c));
        map.boxes_by_loc.remove(&(r, c));
        if matches!(part, Part::P2) {
            map.boxes_by_loc.remove(&(r, c + 1));
        }
    }

    for (r, c) in to_move {
        let (nr, nc) = dest(&(r, c), dir);
        map.boxes.insert((nr, nc));
        map.boxes_by_loc.insert((nr, nc), (nr, nc));
        if matches!(part, Part::P2) {
            map.boxes_by_loc.insert((nr, nc + 1), (nr, nc));
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (mut map, moves) = parse_input(stdin().lock(), &args.part)?;
    for dir in moves {
        let robot_dest = dest(&map.robot, &dir);
        if let Some(to_move) = boxes_to_move(&map, robot_dest, &dir, &args.part) {
            move_all_boxes(&mut map, to_move, &dir, &args.part);
            map.robot = robot_dest;
        }
    }

    println!(
        "{}",
        map.boxes
            .iter()
            .map(|&(r, c)| { 100 * r + c })
            .sum::<usize>()
    );

    Ok(())
}
