use std::collections::HashMap;

//use clap::Parser;
use itertools::Itertools;

use aoc_2022::{Handler, Plugin};

fn main() {
    let solutions = inventory::iter::<Plugin>()
        .map(|p| (p.0, p.1))
        .collect::<HashMap<&str, Handler>>();
    println!("Solutions found: {}", solutions.keys().join(" "));
    //let mut param = "1 2 3 4".split_whitespace();
    //println!("{}", tasks.get("print").unwrap()(& mut param));
}
