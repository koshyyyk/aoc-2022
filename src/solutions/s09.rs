use std::cmp::max;
use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

use crate::{InputIterator, Ztr};

static _TEST_DATA: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

type Coord = i32;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct State{
    head: (Coord, Coord),
    tail: (Coord, Coord),
}

fn mv(&(r, c): &(Coord, Coord), dir: char) -> (Coord, Coord) {
    match dir {
        'R' => (r, c + 1),
        'L' => (r, c - 1),
        'U' => (r + 1, c),
        'D' => (r - 1, c),
        '↗' => (r + 1, c + 1),
        '↘' => (r - 1, c + 1),
        '↙' => (r - 1, c - 1),
        '↖' => (r + 1, c - 1),
        _ => panic!("unhadled direction {}", dir)
    }
}

impl State {

    fn new() -> Self {
        Default::default()
    }

    fn distance(&self) -> i32 {
        max(num::abs(self.head.0 - self.tail.0), num::abs(self.head.1 - self.tail.1))
    }

    fn diagonal(&self) -> bool {
         self.head.0 != self.tail.0 && self.head.1 != self.tail.1
    }

    fn step(mut self, dir: char) -> Self {
        self.head = mv(&self.head, dir);
        if self.distance() > 1 {
            if self.diagonal() {
                let rd = num::signum(self.head.0 - self.tail.0);
                let cd = num::signum(self.head.1 - self.tail.1);
                self.tail = (self.tail.0 + rd, self.tail.1 + cd);
            } else {
                self.tail = mv(&self.tail, dir);
            }
        }
        self
    }

}

fn read_input(it: InputIterator) -> impl Iterator<Item = char> + '_ {
    it.flat_map(|line| {
        let mut components = line.split_whitespace().collect::<Vec<_>>();
        let count = components.pop().and_then(|num| usize::from_str(num).ok()).unwrap_or_default();
        repeat(components[0].chars().next().unwrap()).take(count)
    })
}

#[test]
fn test_read() {
    let read = read_input(& mut (_TEST_DATA.lines().map(|s| s.into()))).collect::<Vec<_>>();
    println!("{:?}", read)
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut trace: HashSet<(Coord, Coord)> = HashSet::new();
    read_input(it).fold(State::new(), |state, dir| {
        trace.insert(state.tail);
        state.step(dir)});
    (if part_two {
        todo!()
    } else {
        trace.len() + 1 // off by one, trace.insert before step
    }).to_string().into()
}

#[test]
fn test1() {
    assert_eq!("13", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
}

#[test]
fn test2() {
    assert!(true);
}
