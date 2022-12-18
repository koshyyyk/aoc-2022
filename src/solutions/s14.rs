use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter::once;

use pom::parser::{list, Parser, seq, sym};

use crate::{InputIterator, Ztr};
use crate::parsers::integer;

static _TEST_DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

type Coord = (i32, i32);

fn parse_line<'a>() -> Parser<'a, u8, Vec<Coord>> {
    let tuple = integer() - sym(b',').discard() + integer();
    list(tuple, seq(b" -> "))
}

fn create_rocks(points: &[Coord], world: &mut HashSet<Coord>) {
    let mut prev: Option<&Coord> = None;
    for point in points.iter() {
        if let Some(prev) = prev {
            match (prev, point) {
                ((c1, r1), (c2, r2)) if *r1 == *r2 => {
                        (min(*c1, *c2)..=max(*c1, *c2)).for_each(|c| {
                            world.insert((c, *r1));
                        })
                    },
                ((c1, r1), (c2, r2)) if *c1 == *c2 => {
                        (min(*r1, *r2)..=max(*r1, *r2)).for_each(|r| {
                            world.insert((*c1, r));
                        })
                },
                _ => panic!("but why?")
            }
        }
        prev = Some(point);
    }
}

fn advance((c, r): Coord, world: &mut HashSet<Coord>, bottom: i32) -> bool {
    if r > bottom {
        return true;
    }
    if !world.contains(&(c, r + 1)) {
        advance((c, r + 1), world, bottom)
    } else if !world.contains(&(c - 1, r + 1)) {
        advance((c - 1, r + 1), world, bottom)
    } else if !world.contains(&(c + 1, r + 1)) {
        advance((c + 1, r + 1), world, bottom)
    } else {
        world.insert((c, r));
        false
    }
}

fn advance2((c, r): Coord, world: &mut HashSet<Coord>, bottom: i32) -> bool {
    if r + 1 == bottom {
        world.insert((c, r));
        return false;
    }
    if !world.contains(&(c, r + 1)) {
        advance2((c, r + 1), world, bottom)
    } else if !world.contains(&(c - 1, r + 1)) {
        advance2((c - 1, r + 1), world, bottom)
    } else if !world.contains(&(c + 1, r + 1)) {
        advance2((c + 1, r + 1), world, bottom)
    } else {
        if world.contains(&(c, r)) {
            true
        } else {
            world.insert((c, r));
            false
        }
    }
}

fn main_loop<A>(mut advancer: A) -> usize
where A: FnMut(Coord) -> bool {
    once((500, 0)).cycle()
        .enumerate()
        .map(|(n, coord)| (n, advancer(coord)))
        .find(|&(_, res)| res)
        .map(|(n, _)| n)
        .unwrap_or_default()
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let lines = it.collect::<Vec<_>>();
    let parser = parse_line();
    let mut world: HashSet<Coord> = HashSet::new();
    lines.iter().for_each(|line| {
        if let Ok(coords) = parser.parse(line.as_bytes()) {
            create_rocks(&coords, &mut world);
        }
    });
    let bottom = world.iter().map(|&c| c.1).max().unwrap();
    (if part_two {
        main_loop(|coord| advance2(coord, &mut world, bottom + 2))
    } else {
        main_loop(|coord| advance(coord, &mut world, bottom))
    }).to_string().into()
}

#[test]
fn test1() {
    assert_eq!("24", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
}

#[test]
fn test2() {
    assert_eq!("93", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true));
}
