use std::io::{read_to_string, stdin, BufRead};

use anyhow::Result;
use clap::{Parser, Subcommand};
use nom::{
    bytes::complete::tag,
    character::{
        complete::{digit1, line_ending},
        streaming::one_of,
    },
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    Err, IResult,
};
use z3::{
    ast::{Ast, Int},
    Config, Context, Optimize, SatResult,
};

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

#[derive(Debug)]
struct Vec2 {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct ClawMachine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn num(s: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(s)
}

fn button_line(s: &str) -> IResult<&str, Vec2> {
    let (s, _) = tag("Button ")(s)?;
    let (s, _) = one_of("AB")(s)?;
    let (s, _) = tag(": X+")(s)?;
    let (s, x) = num(s)?;
    let (s, _) = tag(", Y+")(s)?;
    let (s, y) = num(s)?;
    let (s, _) = line_ending(s)?;

    Ok((s, Vec2 { x, y }))
}

fn prize_line(s: &str) -> IResult<&str, Vec2> {
    let (s, _) = tag("Prize: X=")(s)?;
    let (s, x) = num(s)?;
    let (s, _) = tag(", Y=")(s)?;
    let (s, y) = num(s)?;
    let (s, _) = line_ending(s)?;

    Ok((s, Vec2 { x, y }))
}

fn claw_machine(s: &str) -> IResult<&str, ClawMachine> {
    let (s, a) = button_line(s)?;
    let (s, b) = button_line(s)?;
    let (s, prize) = prize_line(s)?;

    Ok((s, ClawMachine { a, b, prize }))
}

fn claw_machines(inp: impl BufRead) -> Result<Vec<ClawMachine>> {
    let s = read_to_string(inp)?;
    let (_, res) =
        separated_list1(line_ending, claw_machine)(&s).map_err(Err::<Error<&str>>::to_owned)?;
    Ok(res)
}

fn min_tokens(machine: &ClawMachine, offset: u64) -> Option<u64> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let a_x = Int::from_u64(&ctx, machine.a.x);
    let a_y = Int::from_u64(&ctx, machine.a.y);
    let b_x = Int::from_u64(&ctx, machine.b.x);
    let b_y = Int::from_u64(&ctx, machine.b.y);
    let prize_x = Int::from_u64(&ctx, machine.prize.x + offset);
    let prize_y = Int::from_u64(&ctx, machine.prize.y + offset);

    let a_n = Int::new_const(&ctx, "a_n");
    let b_n = Int::new_const(&ctx, "b_n");

    let zero = Int::from_u64(&ctx, 0);

    let optimize = Optimize::new(&ctx);

    // a_n >= 0, b_n >= 0
    optimize.assert(&a_n.ge(&zero));
    optimize.assert(&b_n.ge(&zero));

    // a_x * a_n + b_x * b_n = prize_x
    let x_a = Int::mul(&ctx, &[&a_x, &a_n]);
    let x_b = Int::mul(&ctx, &[&b_x, &b_n]);
    let x = Int::add(&ctx, &[&x_a, &x_b]);
    optimize.assert(&x._eq(&prize_x));

    // a_y * a_n + b_y * b_n = prize_y
    let y_a = Int::mul(&ctx, &[&a_y, &a_n]);
    let y_b = Int::mul(&ctx, &[&b_y, &b_n]);
    let y = Int::add(&ctx, &[&y_a, &y_b]);
    optimize.assert(&y._eq(&prize_y));

    // minimize total tokens
    let tokens = Int::add(&ctx, &[&a_n, &b_n]);
    optimize.minimize(&tokens);

    match optimize.check(&[]) {
        SatResult::Sat => {
            let model = optimize.get_model().unwrap();
            let a = model.get_const_interp(&a_n).unwrap().as_u64()?;
            let b = model.get_const_interp(&b_n).unwrap().as_u64()?;

            Some(3 * a + b)
        }
        _ => None,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let machines = claw_machines(stdin().lock())?;

    let offset = match args.part {
        Part::P1 => 0,
        Part::P2 => 10_000_000_000_000,
    };

    println!(
        "{}",
        machines
            .into_iter()
            .filter_map(|machine| min_tokens(&machine, offset))
            .sum::<u64>()
    );

    Ok(())
}
