use std::{
    collections::HashSet,
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
    robot: Coord,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(inp: impl BufRead) -> Result<(Map, Vec<Direction>)> {
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = None;
    let mut lines = inp.lines().enumerate();
    for (r, line) in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (c, ch) in line.char_indices() {
            match ch {
                '.' => {
                    continue;
                }
                '#' => {
                    walls.insert((r, c));
                }
                'O' => {
                    boxes.insert((r, c));
                }
                '@' => {
                    robot = Some((r, c));
                }
                _ => bail!("Unrecognized character in input: {ch}")
            };
        }
    }

    let robot = robot.ok_or_else(|| anyhow!("Missing robot in input"))?;
    let map = Map {
        walls,
        boxes,
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
                _ => bail!("Unrecognized character in input: {ch}")
            };

            moves.push(mv);
        }
    }

    Ok((map, moves))
}

fn dest(&(r, c): &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => (r-1, c),
        Direction::Down => (r+1, c),
        Direction::Left => (r, c-1),
        Direction::Right => (r, c+1),
    }
}

fn try_clear(map: &mut Map, coord: Coord, dir: Direction) -> bool {
    if map.walls.contains(&coord) {
        false
    } else if map.boxes.contains(&coord) {
        let box_dest = dest(&coord, &dir);
        if try_clear(map, box_dest, dir) {
            map.boxes.remove(&coord);
            map.boxes.insert(box_dest);

            true
        } else {
            false
        }
    } else {
        true
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (mut map, moves) = parse_input(stdin().lock())?;

    for mv in moves {
        let robot_dest = dest(&map.robot, &mv);
        if try_clear(&mut map, robot_dest, mv) {
            map.robot = robot_dest;
        }
    }

    match args.part {
        Part::P1 => println!("{}", map.boxes.iter().map(|&(r, c)| 100 * r + c).sum::<usize>()),
        Part::P2 => todo!(),
    }

    Ok(())
}
