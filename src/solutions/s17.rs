use std::cmp;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::{InputIterator, Ztr};

trait Shape {
    // type World;
    fn left(&self) -> i64;
    fn right(&self) -> i64;
    fn top(&self) -> i64;
    fn bottom(&self) -> i64;
}

type Coord = (i64, i64); // row, column

#[derive(Debug, Clone, PartialEq, Eq)]
struct VecShape(Vec<Coord>);

impl Shape for VecShape {
    fn left(&self) -> i64 {
        self.0.iter().map(|&(_, c)| c).min().unwrap_or_default()
    }

    fn right(&self) -> i64 {
        self.0.iter().map(|&(_, c)| c).max().unwrap_or_default()
    }

    fn top(&self) -> i64 {
        self.0.iter().map(|&(r, _)| r).max().unwrap_or_default()
    }

    fn bottom(&self) -> i64 {
        self.0.iter().map(|&(r, _)| r).min().unwrap_or_default()
    }

}

// Each rock appears so that its left edge is two units away from the left wall
// and its bottom edge is three units above the highest rock in the room
// (or the floor, if there isn't one)
impl VecShape {
    pub fn at_height(index: usize, r: i64) -> VecShape {
        match index {
            0 => VecShape(vec![(r, 2), (r, 3), (r, 4), (r, 5)]),                            // ####

                                                                                            //  #
            1 => VecShape(vec![(r, 3), (r + 1, 2), (r + 1, 3), (r + 1, 4), (r + 2, 3)]),    // ###
                                                                                            //  #

                                                                                            //   #
                                                                                            //   #
            2 => VecShape(vec![(r, 2), (r, 3), (r, 4), (r + 1, 4), (r + 2, 4)]),            // ###

                                                                                            // #
                                                                                            // #
                                                                                            // #
            3 => VecShape(vec![(r, 2), (r + 1, 2), (r + 2, 2), (r + 3, 2)]),                // #

                                                                                            // ##
            4 => VecShape(vec![(r, 2), (r, 3), (r + 1, 2), (r + 1, 3)]),                    // ##

            _ => panic!("eehh")
        }
    }

    pub fn push(& mut self, dir: char, world: &HashSet<Coord>) {
        let old = self.clone();
        match dir {
            '<' if self.left() > 0 => self.0.iter_mut().for_each(|c| c.1 -= 1),
            '>' if self.right() < 6 => self.0.iter_mut().for_each(|c| c.1 += 1),
            _ => ()
        };
        if self.collides(world) {
            *self = old;
        }
        //println!("after pushing {}: {:?}", dir, self);
    }

    pub fn collides(&self, world: &HashSet<Coord>) -> bool {
        self.0.iter().any(|c| world.contains(c))
    }

    pub fn down(& mut self, world: &mut HashSet<Coord>) -> Option<i64> {
        let mut new = self.clone();
        new.0.iter_mut().for_each(|c| c.0 -= 1);
        let r = if new.bottom() == 0 || new.collides(world) {
            self.0.iter().for_each(|&c| { world.insert(c); });
            //let waterline = self.top() - 20;
            Some(self.top())
        } else {
            *self = new;
            None
        };
        //println!("after down: {:?}", self);
        r
    }

    pub fn step(& mut self, dir: char, world: &mut HashSet<Coord>) -> Option<i64> {
        self.push(dir, world);
        self.down(world)
    }

}

#[allow(unused)]
fn print_world(world: &HashSet<Coord>) {
    let top = world.iter().map(|&(r, _)| r).max().unwrap_or_default();
    for r in (1..=top).rev() {
        let line: String = (0..7).map(|c| if world.contains(&(r, c)) {'#'} else { '.' }).collect();
        println!("{:04} |{}|", r, line);
    }
    println!("0000 +-------+");
}

fn find_period<T: Ord>(inp: &[T]) -> Option<usize> {
    for offset in 1..=(inp.len() / 2) {
        let i1 = inp.into_iter();
        let i2 = inp.into_iter().skip(offset);
        let m = i1.zip(i2).map(|(c1, c2)| c1.cmp(c2)).all(|r| r == Ordering::Equal);
        if m {
            return Some(offset)
        }
    }
    None
}

#[test]
fn any_period() {
    assert_eq!(Some(6), find_period(&vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, ]));
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut world: HashSet<Coord> = HashSet::new();
    let dir_vec = it.next().iter().flat_map(|l| l.chars()).collect::<Vec<_>>();
    let mut directions = dir_vec.iter().cycle();
    let rocks = if part_two { 1000000000000 } else { 2022 };
    let r = (0..5).cycle()
        .take(rocks)
        .enumerate()
        .fold(0_i64, |last_top, (_, n)| {
            //println!("last top {}", last_top);
            let mut shape = VecShape::at_height(n, last_top + 4);
            let top = directions.by_ref()
                .map(|&dir| shape.step(dir, &mut world))
                .skip_while(|r| r.is_none())
                .next()
                .flatten()
                .unwrap_or_default();
            cmp::max(top, last_top)
        });
    //print_world(&world);
    r.to_string().into()
}

#[cfg(test)]
mod test {
    use super::*;

    static _TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1() {
        assert_eq!("3068", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

// #[test]
// fn test2() {
//     assert_eq!("1514285714288", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true))
// }
}
