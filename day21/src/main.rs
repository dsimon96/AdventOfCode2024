use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

use anyhow::{bail, Context, Result};
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
const NUMPAD_A: Coords = (3, 2);

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
const DIRKEY_A: Coords = (0, 2);

type Cursors = [Coords; 3];

const CURSORS_INIT: Cursors = [DIRKEY_A, DIRKEY_A, NUMPAD_A];

const fn try_move(&(r, c): &Coords, dir: Direction, height: usize, width: usize) -> Option<Coords> {
    match dir {
        Direction::Up if r > 0 => Some((r - 1, c)),
        Direction::Down if r < height - 1 => Some((r + 1, c)),
        Direction::Left if c > 0 => Some((r, c - 1)),
        Direction::Right if c < width - 1 => Some((r, c + 1)),
        _ => None,
    }
}

fn do_input(cursors: &Cursors, mut input: Key) -> Option<(Cursors, Option<Button>)> {
    let mut new_cursors = *cursors;
    for i in 0..cursors.len() {
        let (height, width) = if i == cursors.len() - 1 {
            (NUMPAD_HEIGHT, NUMPAD_WIDTH)
        } else {
            (DIRKEYS_HEIGHT, DIRKEYS_WIDTH)
        };
        match input {
            Key::Dir(dir) => match (i, try_move(&cursors[i], dir, height, width)) {
                (i, Some((nr, nc)))
                    if (i < cursors.len() - 1 && DIRKEYS[nr][nc].is_some()
                        || i == cursors.len() - 1 && NUMPAD[nr][nc].is_some()) =>
                {
                    new_cursors[i] = (nr, nc);
                    return Some((new_cursors, None));
                }
                _ => {
                    return None;
                }
            },
            Key::A => {
                let (r, c) = cursors[i];
                if i < cursors.len() - 1 {
                    input = DIRKEYS[r][c].unwrap();
                } else {
                    return Some((new_cursors, NUMPAD[r][c]));
                }
            }
        }
    }

    unreachable!()
}

fn search(cursors: &Cursors, target: Button) -> Result<(Vec<Key>, Cursors)> {
    let mut visited = HashSet::from([*cursors]);
    let mut q: VecDeque<(Vec<Key>, Cursors)> = VecDeque::from([(Vec::new(), *cursors)]);

    loop {
        let (keys, cursors) = q.pop_front().context("Didn't find path")?;
        for input in [
            Key::Dir(Direction::Up),
            Key::Dir(Direction::Down),
            Key::Dir(Direction::Left),
            Key::Dir(Direction::Right),
            Key::A,
        ] {
            let Some((new_cursors, button)) = do_input(&cursors, input) else {
                continue;
            };
            let mut new_keys = keys.clone();
            new_keys.push(input);

            match button {
                Some(pressed) if pressed == target => return Ok((new_keys, new_cursors)),
                None if visited.insert(new_cursors) => q.push_back((new_keys, new_cursors)),
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut total = 0;
    for s in stdin().lines() {
        let s = s?;
        let numeric_part = s[..3].parse::<usize>()?;
        let mut cursors = CURSORS_INIT;
        let mut shortest_len = 0;
        for ch in s.chars() {
            let button = if let Some(num) = ch.to_digit(10) {
                Button::Num(num.try_into()?)
            } else if ch == 'A' {
                Button::A
            } else {
                bail!("Invalid character in input");
            };

            let (moves, new_cursors) = search(&cursors, button)?;
            shortest_len += moves.len();
            cursors = new_cursors;
        }

        total += shortest_len * numeric_part;
    }

    match args.part {
        Part::P1 => println!("{total}"),
        Part::P2 => todo!(),
    }

    Ok(())
}
