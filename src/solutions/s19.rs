use std::str::FromStr;

use regex::{Captures, Regex};

use crate::{InputIterator, Ztr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Blueprint {
    id: i32,
    ore: i32,
    clay: i32,
    obs: (i32, i32),
    geo: (i32, i32)
}

impl Blueprint {
    fn new(id: i32, ore: i32, clay: i32, o1: i32, o2: i32, g1: i32, g2: i32 ) -> Blueprint {
        Blueprint { id, ore, clay, obs: (o1, o2), geo: (g1, g2) }
    }

    fn from_captures(captures: &Captures) -> Option<Blueprint> {
        if captures.len() == 8 {
            let params = captures.iter()
                    .skip(1)
                    .flat_map(|c| c.into_iter())
                    .flat_map(|m| i32::from_str(m.as_str()))
                    .collect::<Vec<_>>();
            Some(Blueprint {
                id: params[0],
                ore: params[1],
                clay: params[2],
                obs: (params[3], params[4]),
                geo: (params[5], params[6])
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct World {
    ore: i32,
    clay: i32,
    ore_r: i32,
    clay_r: i32,
    obs_r: i32,
    geo_r: i32,
}

impl World {
    fn new() -> World {
        World { ore: 0, clay: 0, ore_r: 1, clay_r: 0, obs_r: 0, geo_r: 0 }
    }

}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let costs = Regex::new(r".+(\d+).+(\d+).+(\d+).+(\d+).+(\d+).+(\d+).+(\d+)").unwrap();
    let blueprints = it.flat_map(|line| {
            costs.captures(&line).and_then(|c| Blueprint::from_captures(&c))
        })
        .collect::<Vec<_>>();
    if part_two {
        todo!()
    } else {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    static _TEST_DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
    #[test]
    fn test1() {
        assert!(true);
    }

    #[test]
    fn test2() {
        assert!(true);
    }

}
