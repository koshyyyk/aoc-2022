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

static _TEST_DATA2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

type Coord = i32;

#[derive(Debug, Eq, PartialEq)]
struct State{
    rope: Vec<(Coord, Coord)>
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

    fn new(len: usize) -> Self {
        State{rope: repeat((0, 0)).take(len).collect()}
    }

    fn head(&mut self) -> &mut (Coord, Coord) {
        &mut self.rope[0]
    }

    fn tail(&mut self) -> &mut (Coord, Coord) {
        let len = self.rope.len();
        &mut self.rope[len - 1]
    }

    fn distance(&self, knot: usize) -> i32 {
        max(num::abs(self.rope[knot].0 - self.rope[knot - 1].0),
            num::abs(self.rope[knot].1 - self.rope[knot - 1].1))
    }

    fn adjust_knot(&mut self, knot: usize) {
        let ahead = knot - 1;
        if self.distance(knot) > 1 {
            let rd = num::signum(self.rope[ahead].0 - self.rope[knot].0);
            let cd = num::signum(self.rope[ahead].1 - self.rope[knot].1);
            self.rope[knot] = (self.rope[knot].0 + rd, self.rope[knot].1 + cd);
        }
    }

    fn step(&mut self, dir: char) {
        *self.head() = mv(self.head(), dir);
        (1..self.rope.len()).for_each(|knot| self.adjust_knot(knot));
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
    let len = if part_two {
        10
    } else {
        2
    };
    let mut state = State::new(len);
    read_input(it).enumerate().for_each(|(_n, dir)| {
        state.step(dir);
        trace.insert(*state.tail());
        //println!("{:03} {} {:?}", _n, dir, &state.rope);
    });
    trace.len().to_string().into()
}

#[test]
fn test1() {
    assert_eq!("13", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
}

#[test]
fn test21() {
    assert_eq!("1", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true));
}

#[test]
fn test22() {
    assert_eq!("36", solution(& mut (_TEST_DATA2.lines().map(|s| s.into())), true));
}
