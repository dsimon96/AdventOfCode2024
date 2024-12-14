use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use nom::{
    character::{
        complete::{char, digit1, space1},
        streaming::one_of,
    },
    combinator::{map_res, opt, recognize},
    error::Error,
    sequence::{preceded, separated_pair},
    Err, IResult,
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
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn num(s: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(char('-')), digit1)), str::parse)(s)
}

fn vec2(s: &str) -> IResult<&str, Vec2> {
    let (s, (x, y)) = separated_pair(num, char(','), num)(s)?;
    Ok((s, Vec2 { x, y }))
}

fn component(s: &str) -> IResult<&str, Vec2> {
    let (s, (_, vec)) = separated_pair(one_of("pv"), char('='), vec2)(s)?;
    Ok((s, vec))
}

fn robot(s: &str) -> IResult<&str, Robot> {
    let (s, (pos, vel)) = separated_pair(component, space1, component)(s)?;

    Ok((s, Robot { pos, vel }))
}

fn determine_pos(robot: &Robot, width: usize, height: usize, steps: usize) -> Vec2 {
    let x = (robot.pos.x + i64::try_from(steps).unwrap() * robot.vel.x)
        .rem_euclid(i64::try_from(width).unwrap());
    let y = (robot.pos.y + i64::try_from(steps).unwrap() * robot.vel.y)
        .rem_euclid(i64::try_from(height).unwrap());

    Vec2 { x, y }
}

#[derive(PartialEq, Eq, Hash)]
enum Half {
    Lower,
    Upper,
}

type Quadrant = (Half, Half);

fn half(pos: i64, len: usize) -> Option<Half> {
    let threshold = i64::try_from(len).unwrap() / 2;
    if len % 2 == 1 && pos == threshold {
        None
    } else if pos < threshold {
        Some(Half::Lower)
    } else {
        Some(Half::Upper)
    }
}

fn quadrant(pos: &Vec2, width: usize, height: usize) -> Option<Quadrant> {
    match (half(pos.x, width), half(pos.y, height)) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

fn calc_safety_factor(robots: &Vec<Robot>, width: usize, height: usize, steps: usize) -> usize {
    let mut quadrant_counts: HashMap<Quadrant, usize> = HashMap::new();
    for robot in robots {
        let pos = determine_pos(robot, width, height, steps);

        if let Some(quadrant) = quadrant(&pos, width, height) {
            *quadrant_counts.entry(quadrant).or_default() += 1;
        }
    }

    quadrant_counts.values().product()
}

fn visualize_robots(robots: &Vec<Robot>, width: usize, height: usize, steps: usize) {
    let mut count_by_pos: HashSet<Vec2> = HashSet::new();
    for robot in robots {
        let pos = determine_pos(robot, width, height, steps);
        count_by_pos.insert(pos);
    }

    for y in 0..height {
        println!(
            "{}",
            (0..width)
                .map(|x| {
                    let y = i64::try_from(y).unwrap();
                    let x = i64::try_from(x).unwrap();
                    if count_by_pos.contains(&Vec2 { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let robots = stdin()
        .lines()
        .map(|line| -> Result<Robot> {
            let line = line?;
            let (_, robot) = robot(&line).map_err(Err::<Error<&str>>::to_owned)?;
            Ok(robot)
        })
        .collect::<Result<Vec<Robot>>>()?;

    match args.part {
        Part::P1 => println!(
            "{}",
            calc_safety_factor(&robots, args.width, args.height, 100)
        ),
        Part::P2 => {
            let search_ub = 10000;
            let mut results = (0..search_ub)
                .map(|steps| {
                    (
                        calc_safety_factor(&robots, args.width, args.height, steps),
                        steps,
                    )
                })
                .collect::<Vec<_>>();
            results.sort_unstable();

            let &(_, steps) = results.first().unwrap();
            visualize_robots(&robots, args.width, args.height, steps);
            println!("{steps}");
        }
    }

    Ok(())
}
