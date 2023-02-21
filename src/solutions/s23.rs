use std::cell::Cell;
use std::collections::HashMap;

use num::abs;

use Direction::*;

use crate::{Coord, InputIterator, Ztr};

static _TEST_DATA: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";

static _TEST_DATA_2: &str = ".....
..##.
..#..
.....
..##.
.....
";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    W,
    E
}

impl Coord {

    fn in_sight(&self, other: &Self, dir: Direction) -> bool {
        match dir {
            N if self.row - 1 == other.row && abs(self.col - other.col) < 2
                => true,
            S if self.row + 1 == other.row && abs(self.col - other.col) < 2
                => true,
            W if self.col - 1 == other.col && abs(self.row - other.row) < 2
                => true,
            E if self.col + 1 == other.col && abs(self.row - other.row) < 2
                => true,
            _ => false
        }
    }

    fn make_prop(&self, dir: Direction, others: &mut dyn Iterator<Item = &Self>) -> Option<Self> {
        let neigh = others.filter(|other| abs(self.col - other.col) < 2 && abs(self.row - other.row) < 2)
                                              .collect::<Vec<_>>();
        if neigh.len() == 1 {
            return None
        }
        for other in neigh {
            if self.in_sight(other, dir) {
                return None
            }
        }
        match dir {
            N =>
                Some(Coord { row: self.row - 1, ..*self }),
            S =>
                Some(Coord { row: self.row + 1, .. *self }),
            W =>
                Some(Coord { col: self.col - 1, .. *self }),
            E =>
                Some(Coord { col: self.col + 1, .. *self }),
        }
    }

}

#[derive(Debug, Eq, PartialEq)]
struct Elf {
    current: Coord,
    prop: Cell<Option<Coord>>
}

impl Elf {

    fn new(row: i32, column: i32) -> Self {
        Elf { current: Coord { row, col: column }, prop: Cell::new(None) }
    }

    fn propose(&self, directions: &[Direction], others: &[Elf]) -> Option<Coord> {
        let new_pos = directions.iter()
                                .map(|&d| {
                                    let mut coords = others.iter().map(|e| &e.current);
                                    self.current.make_prop(d, &mut coords)
                                })
                                .skip_while(|&c| c.is_none())
                                .next()
                                .flatten();
        self.prop.set(new_pos);
        new_pos
    }

    fn rollback(&self) {
        self.prop.set(None);
    }

    fn mv(&self) -> Self {
        if let Some(prop) = self.prop.take() {
            //println!("moving ({},{}) to ({},{})", self.current.row, self.current.column, prop.row, prop.column);
            Elf { current: prop, prop: Cell::new(None) }
        } else {
            Elf { prop: Cell::new(self.prop.get()), .. *self }
        }
    }

}

fn step(elves: &[Elf], dirs: &mut dyn Iterator<Item = Direction>) -> (Vec<Elf>, bool) {
    let mut dirz = dirs.take(5).collect::<Vec<_>>();
    dirz.pop();
    let mut collisions: HashMap<Coord, usize> = HashMap::new();
    for idx in 0..elves.len() {
        if let Some(coord) = elves[idx].propose(&dirz, elves) {
            collisions.entry(coord).and_modify(|i| {
                    elves[*i].rollback();
                    elves[idx].rollback();
                    *i = idx;
                })
                .or_insert(idx);
        }
    }
    let no_moves = elves.iter().all(|e| e.prop.get().is_none());
    (elves.iter().map(Elf::mv).collect::<Vec<_>>(), no_moves)
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut elves: Vec<Elf> = Vec::with_capacity(1000);
    it.enumerate()
      .for_each(|(row, line)| line.chars()
                                              .enumerate()
                                              .filter(|(_, c)| *c == '#')
                                              .for_each(|(col, _)| elves.push(Elf::new(row as i32, col as i32))));
    let mut dirs = vec![N, S, W, E].into_iter().cycle();
    if part_two {
        let mut done = false;
        let mut c = 0;
        while !done {
            let r = step(&mut elves, &mut dirs);
            elves = r.0;
            done = r.1;
            c += 1;
        }
        c.to_string().into()
    } else {
        for _ in 0..10 {
            elves = step(&mut elves, &mut dirs).0;
        }
        let min_col = elves.iter().map(|e| e.current.col).min().unwrap();
        let max_col = elves.iter().map(|e| e.current.col).max().unwrap();
        let max_row = elves.iter().map(|e| e.current.row).max().unwrap();
        let min_row = elves.iter().map(|e| e.current.row).min().unwrap();
        let w = max_col - min_col + 1;
        let h = max_row - min_row + 1;
        //println!("{} {} {} {}", min_row, max_row, min_col, max_col);
        println!("h {} w {}", h, w);
        (h * w - elves.len() as i32).to_string().into()
    }
}

mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("110", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

    #[test]
    fn test2() {
        assert_eq!("20", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), true));
    }

}
