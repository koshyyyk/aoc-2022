use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;

use itertools::Itertools;

use crate::{Coord, InputIterator, Ztr};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Blizzard {
    dir: char,
    coord: Coord,
}

impl Blizzard {

    fn new(dir: char, row: i32, col: i32) -> Self {
        Blizzard { dir, coord: Coord { row, col } }
    }

    fn maybe_new(dir: char, row: i32, col: i32) -> Option<Self> {
        if "<>^v".contains(dir) {
            Some(Blizzard { dir, coord: Coord { row, col } })
        } else {
            None
        }
    }

    fn advance(&self, w: i32, h: i32) -> Self {
        let dir = self.dir;
        match dir {
            '<' if self.coord.col == 1 => Blizzard::new(dir, self.coord.row, w - 1),
            '<' => Blizzard::new(dir, self.coord.row, self.coord.col - 1),
            '>' if self.coord.col == w - 1 => Blizzard::new(dir, self.coord.row, 1),
            '>' => Blizzard { dir, coord: Coord { row: self.coord.row, col: self.coord.col + 1 }},
            '^' if self.coord.row == 1 => Blizzard { dir, coord: Coord { col: self.coord.col, row: h - 1 }},
            '^' => Blizzard { dir, coord: Coord { col: self.coord.col, row: self.coord.row - 1 }},
            'v' if self.coord.row == h - 1 => Blizzard { dir, coord: Coord { col: self.coord.col, row: 1 }},
            'v' => Blizzard { dir, coord: Coord { col: self.coord.col, row: self.coord.row + 1 }},
            _ => panic!("eh but why?")
        }
    }
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct Pos {
    step: i32,
    coord: Coord,
}

impl Pos {

    fn new(row: i32, col: i32) -> Self {
        Pos { step: 0, coord: Coord { row, col } }
    }

    fn advance(&self, dir: char, w: i32, h: i32) -> Option<Self> {
        let step = self.step + 1;
        let col = self.coord.col;
        let row = self.coord.row;
        // special cases for enter && exit
        match dir {
            '<' if col > 1 && row > 0 && row < h => Some(Pos { step, coord: Coord { col: col - 1, row } }),
            '>' if col < w - 1  && row > 0 && row < h => Some(Pos { step, coord: Coord { col: col + 1, row } }),
            '^' => { // can't go up from (0, 1)
                if row > 1 || (col == 1 && row != 0) {
                    Some(Pos { step, coord: Coord { col, row: row - 1 } })
                } else {
                    None
                }
            },
            'v' => { // can't go down from (h, w - 1)
                if row < h - 1 || (col == w - 1 && row != h) {
                    Some(Pos { step, coord: Coord { row: row + 1, col } })
                } else {
                    None
                }
            },
            'w' => Some(Pos { step, coord: self.coord }),
            _ => None
        }
    }
}

struct World {
    w: i32,
    h: i32,
    blizzards: Vec<Blizzard>,
    positions: VecDeque<Pos>,
    occupied: HashSet<Coord>,
}

impl World {

    fn new(it: InputIterator) -> Self {
        let mut w: i32 = 0;
        let mut h = 0;
        let mut blizzards: Vec<Blizzard> = Vec::with_capacity(1000);
        for (row, line) in it.enumerate() {
            h = row as i32;
            for (col, content) in line.chars().enumerate() {
                if w < col as i32 {
                    w = col as i32;
                }
                if let Some(blizzard) = Blizzard::maybe_new(content, row as i32, col as i32) {
                    blizzards.push(blizzard);
                }
            }
        }
        let positions: VecDeque<Pos> = VecDeque::with_capacity(8);
        World { w, h, blizzards, positions, occupied: HashSet::with_capacity(1000) }
    }

    fn step(& mut self, to: &Coord) -> Option<i32> {
        self.occupied.clear();
        self.blizzards = self.blizzards.iter()
            .map(|b| b.advance(self.w, self.h))
            .inspect(|b| { self.occupied.insert(b.coord); })
            .collect();
        self.positions = self.positions.iter().cartesian_product("v><^w".chars())
            .filter_map(|(pos, dir)| pos.advance(dir, self.w, self.h))
            .filter(|p| !self.occupied.contains(&p.coord))
            .unique()
            .collect();
        if self.positions.is_empty() {
            panic!("STUCK");
        }
        if let Some(Pos { step, .. }) = self.positions.iter().filter(|p| p.coord == *to).next() {
            return Some(*step);
        }
        None
    }

    fn run(&mut self, from: &Coord, to: &Coord) -> i32 {
        self.positions.clear();
        self.positions.push_back(Pos::new(from.row, from.col));
        loop {
            if let Some(n) = self.step(to) {
                break n
            }
        }
    }

}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut world = World::new(it);
    let start = Coord { row: 0, col: 1 };
    let finish = Coord { row: world.h, col: world.w - 1 };
    (if part_two {
        let mut n = world.run(&start, &finish);
        n += world.run(&finish, &start);
        n += world.run(&start, &finish);
        n
    } else {
        world.run(&start, &finish)
    }).to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_DATA: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test1() {
        let mut world = World::new(& mut _TEST_DATA.lines().map(|l| l.into()));
        let start = Coord { row: 0, col: 1 };
        let finish = Coord { row: world.h, col: world.w - 1 };
        world.positions.push_back(Pos::new(start.row, start.col));
        //println!("{:?}\n{:?}\n----------", &world.blizzards, &world.positions);
        world.step(&finish);
        world.step(&finish);
        world.step(&finish);
        world.step(&finish);
        assert_eq!(2, world.blizzards.iter().filter(|b| b.coord == Coord { row: 1, col: 5 }).count());
        assert!(world.blizzards.contains(&Blizzard { dir: '<', coord: Coord { row: 3, col: 1 } }));
        assert!(world.blizzards.contains(&Blizzard{ dir: '^', coord: Coord { row: 4, col: 2 }}));
        assert!(world.blizzards.contains(&Blizzard{ dir: '<', coord: Coord { row: 1, col: 2 }}));
        //assert!(world.positions.contains(&Pos{ step: 4, coord: Coord { row: 1, col: 1 }}));
        let n = loop {
            if let Some(n) = world.step(&finish) {
               break n
            }
        };
        assert_eq!(18, n);
    }

    #[test]
    fn test2() {
        let mut world = World::new(& mut _TEST_DATA.lines().map(|l| l.into()));
        let start = Coord { row: 0, col: 1 };
        let goal = Coord { row: world.h, col: world.w - 1 };
        let mut n = world.run(&start, &goal);
        assert_eq!(18, n);
        n = world.run(&goal, &start);
        assert_eq!(23, n);
        n = world.run(&start, &goal);
        assert_eq!(13, n);
    }

}
