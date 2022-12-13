use std::iter::once;
use std::str::FromStr;

use itertools::Either;

use crate::{InputIterator, Ztr};

static _TEST_DATA: &str = "noop
addx 3
addx -5
";

static _TEST_DATA2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

fn read_input(it: InputIterator) -> impl Iterator<Item = i32> + '_ {
    it.flat_map(|line| {
        if line.starts_with("noop") {
            Either::Left(once(0))
        } else {
            let mut components = line.split_whitespace().collect::<Vec<_>>();
            let count = components.pop().and_then(|num| i32::from_str(num).ok()).unwrap_or_default();
            Either::Right(once(0).chain(once(count)))
        }
    })
}

fn one(it: InputIterator, seqs: &[usize]) -> (i32, i32) {
    let mut acc = 1;
    let mut res = 0;
    read_input(it)
        .enumerate()
        .for_each(|(n, inc)| {
            let i = n + 1;
            if seqs.contains(&i) {
                res += (i as i32) * acc;
            }
            acc += inc;
            //println!("{:03} {:03} {:03} {:03}", i, inc, acc, res);
        });
    (acc, res)
}

fn two(it: InputIterator) -> i32 {
    let mut acc = 1_i32;
    read_input(it)
        .enumerate()
        .for_each(|(n, inc)| {
            let i = ((n + 1) % 40) as i32;
            if i >= acc && i < acc + 3 {
                print!("{}", '#')
            } else {
                print!("{}", '.')
            }
            if i % 40 == 0 {
                println!()
            }
            acc += inc;
        });
    acc
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    (if part_two {
        two(it);
        0
    } else {
        let seqs = vec![20, 60, 100, 140, 180, 220];
        one(it, &seqs).1
    }).to_string().into()
}

#[test]
fn test1() {
    assert_eq!(-1, one(& mut (_TEST_DATA.lines().map(|s| s.into())), &vec![]).0)
}

#[test]
fn test15() {
    assert_eq!("13140", solution(& mut (_TEST_DATA2.lines().map(|s| s.into())), false))
}

#[test]
fn test2() {
    assert_eq!(17, two(& mut (_TEST_DATA2.lines().map(|s| s.into()))));
}
