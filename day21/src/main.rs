use anyhow::Result;
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

enum Button {
    Num(usize),
    A
}

const NUMPAD: [[Option<Button>; 3]; 4] = [
    [Some(Button::Num(7)), Some(Button::Num(8)), Some(Button::Num(9))],
    [Some(Button::Num(4)), Some(Button::Num(5)), Some(Button::Num(6))],
    [Some(Button::Num(1)), Some(Button::Num(2)), Some(Button::Num(3))],
    [None, Some(Button::Num(3)), Some(Button::A)]
];

enum Key {
    Up,
    Down,
    Left,
    Right,
    A
}

const DIRKEYS: [[Option<Key>; 3]; 2] = [
    [None, Some(Key::Up), Some(Key::A)],
    [Some(Key::Left), Some(Key::Down), Some(Key::Right)],
];


fn main() -> Result<()> {
    let args = Args::parse();

    match args.part {
        Part::P1 => todo!(),
        Part::P2 => todo!(),
    }

    Ok(())
}
