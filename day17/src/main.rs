use std::{
    io::{stdin, Error},
    num::ParseIntError,
    ops::BitXor,
};

use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand};
use itertools::Itertools;
use z3::{
    ast::{Ast, BV},
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

type Num = u64;

#[derive(Debug, Clone)]
struct Registers {
    a: Num,
    b: Num,
    c: Num,
}

type Program = Vec<Num>;

fn end_of_input() -> anyhow::Error {
    anyhow!("Unexpected end of input")
}

fn parse_register(inp: &mut impl Iterator<Item = Result<String, Error>>) -> Result<Num> {
    Ok(inp
        .next()
        .ok_or_else(end_of_input)??
        .split_whitespace()
        .last()
        .ok_or_else(end_of_input)?
        .parse::<Num>()?)
}

fn parse_program(inp: &mut impl Iterator<Item = Result<String, Error>>) -> Result<Program> {
    Ok(inp
        .next()
        .ok_or_else(end_of_input)??
        .split_whitespace()
        .last()
        .ok_or_else(end_of_input)?
        .split(',')
        .map(str::parse::<Num>)
        .collect::<Result<Vec<Num>, ParseIntError>>()?)
}

fn get_literal(program: &Program, ip: usize) -> Result<Num> {
    let operand = program
        .get(ip)
        .ok_or_else(|| anyhow!("Read operand from invalid pointer"))?;
    Ok(*operand)
}

fn get_combo(registers: &Registers, program: &Program, ip: usize) -> Result<Num> {
    let operand = get_literal(program, ip)?;
    match operand {
        0..=3 => Ok(operand),
        4 => Ok(registers.a),
        5 => Ok(registers.b),
        6 => Ok(registers.c),
        7.. => {
            bail!("Invalid operand {operand}")
        }
    }
}

fn div(registers: &Registers, program: &Program, ip: usize) -> Result<Num> {
    Ok(registers.a / 2u64.pow(get_combo(registers, program, ip)?.try_into()?))
}

fn simulate(
    registers: &mut Registers,
    program: &Program,
    mut out_func: impl FnMut(Num) -> bool,
) -> Result<bool> {
    let mut ip = 0;
    loop {
        let Some(opcode) = program.get(ip) else {
            break;
        };

        ip = match opcode {
            0 => {
                // adv
                registers.a = div(registers, program, ip + 1)?;
                ip + 2
            }
            1 => {
                // bxl
                registers.b = registers.b.bitxor(get_literal(program, ip + 1)?);
                ip + 2
            }
            2 => {
                // bst
                registers.b = get_combo(registers, program, ip + 1)? % 8;
                ip + 2
            }
            3 => {
                // jnz
                if registers.a == 0 {
                    ip + 2
                } else {
                    get_literal(program, ip + 1)?.try_into()?
                }
            }
            4 => {
                // bxc
                registers.b = registers.b.bitxor(registers.c);
                ip + 2
            }
            5 => {
                // out
                if out_func(get_combo(registers, program, ip + 1)? % 8) {
                    return Ok(false);
                }
                ip + 2
            }
            6 => {
                // bdv
                registers.b = div(registers, program, ip + 1)?;
                ip + 2
            }
            7 => {
                // cdv
                registers.c = div(registers, program, ip + 1)?;
                ip + 2
            }
            _ => {
                bail!("Invalid opcode: {opcode}")
            }
        }
    }

    Ok(true)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut inp = stdin().lines();
    let end_of_input = || anyhow!("Unexpected end of input");
    let a = parse_register(&mut inp)?;
    let b = parse_register(&mut inp)?;
    let c = parse_register(&mut inp)?;
    let _ = inp.next().ok_or_else(end_of_input)?;
    let program = parse_program(&mut inp)?;

    let mut registers = Registers { a, b, c };

    match args.part {
        Part::P1 => {
            let mut out = Vec::new();
            simulate(&mut registers, &program, |n| {
                out.push(n);
                false
            })?;
            println!("{}", out.into_iter().map(|n| n.to_string()).join(","));
        }
        Part::P2 => {
            // Solve system of equations based on reverse-engineered input
            let cfg = Config::new();
            let ctx = Context::new(&cfg);
            let opt = Optimize::new(&ctx);

            let init = BV::new_const(&ctx, "init", 64);
            let mut a = init.clone();
            for val in program {
                let b = (a.clone() & 7u64) ^ 1u64;
                let c = a.bvlshr(&b);
                let b = b.bitxor(&c) ^ 4u64;
                opt.assert(&b.extract(2, 0)._eq(&BV::from_u64(&ctx, val, 3)));
                a = a.bvlshr(&BV::from_u64(&ctx, 3, 64));
            }
            opt.assert(&a._eq(&BV::from_u64(&ctx, 0, 64)));
            opt.minimize(&init);

            let SatResult::Sat = opt.check(&[]) else {
                bail!("Failed to solve");
            };

            let init = opt.get_model().unwrap().get_const_interp(&init).unwrap();
            println!("{}", init.as_u64().unwrap());
        }
    }

    Ok(())
}
