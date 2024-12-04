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

fn main() -> Result<()> {
    let args = Args::parse();

    match args.part {
        Part::P1 => todo!(),
        Part::P2 => todo!(),
    }

    Ok(())
}
