use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Error, Result};
use clap::{Parser, ValueEnum};
use itertools::Itertools;

use aoc_2022::{Handler, Plugin};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Part {
    #[default]
    One = 1,
    Two,
}

#[derive(Parser, Debug)]
struct Args {
    /// day of the contest
    day: usize,
    #[arg(value_enum)]
    part: Option<Part>
}

fn main() -> Result<()> {
    let args = Args::try_parse().map_err(|e| Error::from(e))?;
    let solutions = inventory::iter::<Plugin>()
        .map(|p| (p.0, p.1))
        .collect::<HashMap<&str, Handler>>();
    let solution = format!("s{:02}", args.day);
    let part = args.part.unwrap_or_default();
    let handler = (&solutions.get(&solution as &str))
        .ok_or(Error::msg(format!("No solution for day {}", args.day)))?;
    let file = File::open(format!("{:02}.txt", args.day))?;
    let mut it = BufReader::new(file)
        .lines()
        .flat_map(|l| l.ok())
        .map(|l| l.into()) ;
    let res = handler(& mut it, part == Part::Two);
    //format!("Available solutions: {}", solutions.keys().join(" "))
    println!("{}", res);
    Ok(())
}
