use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};

use clap::Parser;
use itertools::Itertools;

use aoc_2022::{Handler, Plugin};

#[derive(Parser, Debug)]
struct Args {
    solution: Option<String>
}


fn main() {
    let solutions = inventory::iter::<Plugin>()
        .map(|p| (p.0, p.1))
        .collect::<HashMap<&str, Handler>>();
    let handler = match Args::parse() {
        Args{ solution: Some(sol)} => solutions.get( &sol as &str),
        _ => None
    };
    let res = handler.map(|h| {
                                    let mut i = BufReader::new(io::stdin())
                                        .lines()
                                        .flat_map(|l| l.ok());
                                    h(& mut i)
                            })
                      .unwrap_or((format!("Available solutions: {}", solutions.keys().join(" ")).into(),
                                         "".into()));
    println!("{} {}", res.0, res.1);
}
