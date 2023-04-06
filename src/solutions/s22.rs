use std::collections::HashSet;
use std::iter::Peekable;
use std::vec::IntoIter;

use pom::parser::{empty, list, one_of, Parser};

use Command::*;
use Facing::*;

use crate::{Coord, InputIterator, Ztr};
use crate::parsers::integer;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Facing {
    R = 0,
    D,
    L,
    U,
}

impl Facing {

    fn advance(&self, cmd: Command) -> Self {
        match cmd {
            RIGHT => match *self {
                U => R,
                D => L,
                L => U,
                R => D
            },
            LEFT => match *self {
                U => L,
                D => R,
                L => D,
                R => U
            },
            FWD => *self
        }
    }

    fn opposite(&self) -> Self {
        match *self {
            U => D,
            D => U,
            L => R,
            R => L,
        }
    }

}

impl Coord {

    fn advance(&self, fac: Facing) -> Self {
        let col = self.col;
        let row = self.row;
        match fac {
            U => Coord { col, row: row - 1 },
            D => Coord { col, row: row + 1 },
            L => Coord { col: col - 1, row },
            R => Coord { col: col + 1, row }
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    ul: Coord,
    lr: Coord,
}

impl Region {
    fn new(ul: Coord) -> Self {
        Region { ul, lr: Coord::default() }
    }

    fn set_lr(&mut self, lr: Coord) {
        self.lr = lr;
    }

    fn is_outside(&self, coord: &Coord) -> bool {
        coord.row < self.ul.row ||
        coord.row > self.lr.row ||
        coord.col < self.ul.col ||
        coord.col > self.lr.col
    }

    fn is_inside(&self, coord: &Coord) -> bool {
        !self.is_outside(coord)
    }

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Command {
    RIGHT,
    LEFT,
    FWD
}

fn steps<'a>() -> Parser<'a, u8, Vec<Command>> {
    integer().map(|n| vec![FWD; n as usize])
}

fn dir<'a>() -> Parser<'a, u8, Vec<Command>> {
    one_of(b"RL").map(|t| {
        vec![match t {
                b'R' => Command::RIGHT,
                b'L' => Command::LEFT,
                _ => panic!("bad bad bad")
            }
        ]
    })
}

fn path<'a>() -> Parser<'a, u8, Vec<Command>> {
    list(steps() | dir(), empty())
        .map(|vv| vv.into_iter().flatten().collect())
}

struct World {
    walls: HashSet<Coord>,
    regions: Vec<Region>,
    fac: Facing,
    pos: Coord,
    commands: Peekable<IntoIter<Command>>
}

fn boundaries(line: &str) -> (i32, i32) {
    let lbound = if line.starts_with(' ') {
                        line.rfind(' ').unwrap_or_default() + 1
                     } else {
                        0
                     };
    let rbound = if line.is_empty() { 0 } else { line.len() as i32 - 1 };
    (lbound as i32, rbound)
}

fn fast_forward(iter: &mut Peekable<IntoIter<Command>>) {
    while let Some(&cmd) = iter.peek() {
        if cmd != FWD {
            return;
        }
        iter.next();
    }
}

impl World {
    fn new(it: InputIterator) -> Self {
        let mut start_col = i32::MAX;
        let mut lbound = i32::MAX;
        let mut rbound = i32::MAX;
        let mut regions = vec![];
        let mut walls: HashSet<Coord> = HashSet::with_capacity(1000);
        for (row, line) in it.enumerate() {
            let (lb, rb) = boundaries(&line);
            if lbound == i32::MAX {
                start_col = lb;
                lbound = lb;
                rbound = rb;
            }
            let region = regions.pop().unwrap_or(Region::new(Coord { row: row as i32, col: lb }));
            regions.push(region);
            if lb != lbound || rb != rbound {
                regions.last_mut().unwrap().set_lr(Coord { col: rbound, row: row as i32 - 1 });
                if line.is_empty() {
                    break
                }
                lbound = lb;
                rbound = rb;
                regions.push(Region::new(Coord { row: row as i32, col: lb }));
            }
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    walls.insert(Coord { row: row as i32, col: col as i32 });
                }
            }
        }
        let commands = if let Some(l) = it.next() {
            path().parse(l.as_bytes()).unwrap()
        } else {
            vec![]
        };
        println!("walls: {:?}, commands: {:?}", walls.len(), commands.len());
        World { regions, walls, commands: commands.into_iter().peekable(), fac: R, pos: Coord { row: 0, col: start_col } }
    }

    fn step(&mut self) -> bool {
        if let Some(cmd) = self.commands.next() {
            //println!("{:?} {:?} {:?}", cmd, self.fac, self.pos);
            self.fac = self.fac.advance(cmd);
            if cmd != FWD {
                return true
            }
            let new_pos = self.pos.advance(self.fac);
            if self.walls.contains(&new_pos) {
                fast_forward(&mut self.commands);
                return true
            }
            if self.regions.iter().any(|r| r.is_inside(&new_pos)) {
                self.pos = new_pos;
                return true
            }
            let opp = self.fac.opposite();
            let mut pos = self.pos;
            loop {
                let pp = pos.advance(opp);
                if self.regions.iter().all(|r| !r.is_inside(&pp)) {
                    if self.walls.contains(&pos) {
                        fast_forward(&mut self.commands);
                        return true
                    }
                    self.pos = pos;
                    return true
                }
                pos = pp;
            }
        } else {
            false
        }
    }

    fn run(&mut self) -> (Facing, Coord) {
        while self.step() {
        }
        (self.fac, self.pos)
    }

}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut w = World::new(it);
    (if part_two {
        todo!()
    } else {
        let (fac, Coord { row, col}) = w.run();
        1000 * (row + 1) + 4 * (col + 1) + fac as i32
    }).to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_DATA: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test1() {
        let r = path().parse(b"4R5L").unwrap();
        assert_eq!(r, vec![FWD, FWD, FWD, FWD, RIGHT, FWD, FWD, FWD, FWD, FWD, LEFT]);
    }

    #[test]
    fn test2() {
        let mut w = World::new(& mut (_TEST_DATA.lines().map(|s| s.into())));
        assert_eq!(13, w.walls.len());
        assert_eq!(3, w.regions.len());
        assert_eq!(Coord { row: 3, col: 11 }, w.regions[0].lr);
        assert_eq!(Coord { row: 8, col: 8 }, w.regions[2].ul);
        assert!(w.step());
        assert_eq!(Coord{ row: 0, col: 9 }, w.pos);
        assert!(w.step());
        assert_eq!(Coord{ row: 0, col: 10 }, w.pos);
        assert!(w.step());
        assert_eq!(Coord{ row: 0, col: 10 }, w.pos);
        for _ in 0..9 {
            w.step();
        }
        //assert_eq!(Coord{ row: 1, col: 10 }, w.pos);
        let (dir, Coord { row, col}) = w.run();
        assert_eq!(6032, 1000 * (row + 1) + 4 * (col + 1) + dir as i32)
    }

}
