use std::{collections::HashMap, io::stdin, iter::repeat_n};

use anyhow::{bail, Result};
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
type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Button {
    Num(usize),
    A,
}

const NUMPAD_HEIGHT: usize = 4;
const NUMPAD_WIDTH: usize = 3;

const NUMPAD: [[Option<Button>; NUMPAD_WIDTH]; NUMPAD_HEIGHT] = [
    [
        Some(Button::Num(7)),
        Some(Button::Num(8)),
        Some(Button::Num(9)),
    ],
    [
        Some(Button::Num(4)),
        Some(Button::Num(5)),
        Some(Button::Num(6)),
    ],
    [
        Some(Button::Num(1)),
        Some(Button::Num(2)),
        Some(Button::Num(3)),
    ],
    [None, Some(Button::Num(0)), Some(Button::A)],
];

fn numpad_coords(button: Button) -> Result<Coords> {
    match button {
        Button::Num(0) => Ok((3, 1)),
        Button::Num(1) => Ok((2, 0)),
        Button::Num(2) => Ok((2, 1)),
        Button::Num(3) => Ok((2, 2)),
        Button::Num(4) => Ok((1, 0)),
        Button::Num(5) => Ok((1, 1)),
        Button::Num(6) => Ok((1, 2)),
        Button::Num(7) => Ok((0, 0)),
        Button::Num(8) => Ok((0, 1)),
        Button::Num(9) => Ok((0, 2)),
        Button::Num(_) => {
            bail!("Invalid input");
        }
        Button::A => Ok((3, 2)),
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Key {
    Dir(Direction),
    A,
}

const DIRKEYS_HEIGHT: usize = 2;
const DIRKEYS_WIDTH: usize = 3;
const DIRKEYS: [[Option<Key>; DIRKEYS_WIDTH]; DIRKEYS_HEIGHT] = [
    [None, Some(Key::Dir(Direction::Up)), Some(Key::A)],
    [
        Some(Key::Dir(Direction::Left)),
        Some(Key::Dir(Direction::Down)),
        Some(Key::Dir(Direction::Right)),
    ],
];
const fn dirkey_coords(key: Key) -> Coords {
    match key {
        Key::Dir(Direction::Up) => (0, 1),
        Key::Dir(Direction::Down) => (1, 1),
        Key::Dir(Direction::Left) => (1, 0),
        Key::Dir(Direction::Right) => (1, 2),
        Key::A => (0, 2),
    }
}

const fn try_move(&(r, c): &Coords, dir: Direction, height: usize, width: usize) -> Option<Coords> {
    match dir {
        Direction::Up if r > 0 => Some((r - 1, c)),
        Direction::Down if r < height - 1 => Some((r + 1, c)),
        Direction::Left if c > 0 => Some((r, c - 1)),
        Direction::Right if c < width - 1 => Some((r, c + 1)),
        _ => None,
    }
}

const fn is_valid(robot_idx: usize, &(r, c): &Coords) -> bool {
    if robot_idx == 0 {
        NUMPAD[r][c].is_some()
    } else {
        DIRKEYS[r][c].is_some()
    }
}

// cost to move robot robot_idx from 'from' to 'target' and then press it,
// given that all robots up to robot_idx start on A
fn search_rec(
    memo: &mut HashMap<(usize, Coords, Coords), usize>,
    num_robot_dirkey: usize,
    robot_idx: usize,
    from: &Coords,
    target: &Coords,
) -> Result<usize> {
    if let Some(&res) = memo.get(&(robot_idx, *from, *target)) {
        return Ok(res);
    }
    if from == target {
        return Ok(1);
    }
    let (height, width) = if robot_idx == 0 {
        (NUMPAD_HEIGHT, NUMPAD_WIDTH)
    } else {
        (DIRKEYS_HEIGHT, DIRKEYS_WIDTH)
    };

    let (fr, fc) = from;
    let (tr, tc) = target;

    let mut moves = Vec::new();
    if fr < tr {
        moves.push((Direction::Down, tr - fr));
    } else if fr > tr {
        moves.push((Direction::Up, fr - tr));
    }
    if fc < tc {
        moves.push((Direction::Right, tc - fc));
    } else if fc > tc {
        moves.push((Direction::Left, fc - tc));
    }

    let paths = moves
        .iter()
        .permutations(moves.len())
        .filter(|permutation| {
            let mut path = Vec::new();
            for (dir, n) in permutation {
                path.extend(repeat_n(dir, *n));
            }

            let mut coords = *from;
            for dir in path {
                coords = try_move(&coords, dir, height, width).unwrap();
                if !is_valid(robot_idx, &coords) {
                    return false;
                }
            }

            true
        });

    let res = if robot_idx == num_robot_dirkey {
        paths
            .map(|path| path.iter().map(|(_, n)| *n).sum::<usize>() + 1)
            .min()
            .unwrap()
    } else {
        let mut path_lens = Vec::new();
        for path in paths {
            let mut path_len = 0;
            let a = dirkey_coords(Key::A);
            let mut coords = a;
            for &(dir, n) in path {
                let new_coords = dirkey_coords(Key::Dir(dir));
                path_len +=
                    search_rec(memo, num_robot_dirkey, robot_idx + 1, &coords, &new_coords)?;
                path_len += n - 1;

                coords = new_coords;
            }
            path_len += search_rec(memo, num_robot_dirkey, robot_idx + 1, &coords, &a)?;
            path_lens.push(path_len);
        }

        path_lens.into_iter().min().unwrap()
    };

    memo.insert((robot_idx, *from, *target), res);
    Ok(res)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let num_robot_dirkey = match args.part {
        Part::P1 => 2,
        Part::P2 => 25,
    };

    let mut total = 0;
    let mut memo = HashMap::new();
    for s in stdin().lines() {
        let s = s?;
        let numeric_part = s[..3].parse::<usize>()?;
        let mut path_len = 0;
        let mut coords = numpad_coords(Button::A)?;
        for ch in s.chars() {
            let button = if let Some(num) = ch.to_digit(10) {
                Button::Num(num.try_into()?)
            } else if ch == 'A' {
                Button::A
            } else {
                bail!("Invalid character in input");
            };

            let new_coords = numpad_coords(button)?;
            let n = search_rec(&mut memo, num_robot_dirkey, 0, &coords, &new_coords)?;
            path_len += n;
            coords = new_coords;
        }

        total += path_len * numeric_part;
    }

    println!("{total}");

    Ok(())
}
