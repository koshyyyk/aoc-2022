use std::collections::HashSet;

use fast_paths::{FastGraph, InputGraph};
use itertools::Itertools;

use crate::{InputIterator, Ztr};

static _TEST_DATA: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

type Coord = (usize, usize);

fn read_to_array<'a>(it: InputIterator, buf: &'a mut Vec<i32>) -> (Vec<&'a [i32]>, Coord, Coord) {
    let mut width = 0_usize;
    let mut start = (0, 0);
    let mut finish = (0, 0);
    it.enumerate().for_each(|(row, ztr)| {
        width = ztr.len();
        buf.append(& mut ztr.chars()
            .enumerate()
            .map(|(col, c)| match c {
                                                'S' => {
                                                    start = (row, col);
                                                    'a'
                                                }
                                                'E' => {
                                                    finish = (row, col);
                                                    'z'
                                                }
                                                _ => c
                                            })
            .map(|c| c as i32).collect::<Vec<_>>());
    });
    (buf.as_slice().chunks(width).collect(), start, finish)
}

fn adj_points((row, col): Coord, h: usize, w: usize) -> impl Iterator<Item = Coord> {
    (-1..=1).cartesian_product(-1..=1)
        .filter(|&(rd, cd)| rd == 0 || cd == 0 && (rd != 0 || cd != 0))
        .map(move |(rd, cd)| ((row as i32) + rd, (col as i32) + cd))
        .filter(move |&(r, c)| r >= 0 && c >= 0 && r < h as i32 && c < w as i32)
        .map(|(r, c)| (r as usize, c as usize))
}

fn adj_map(grid: &[&[i32]]) -> HashSet<(Coord, Coord)> {
    (0..grid.len()).cartesian_product(0..grid[0].len())
        .flat_map(|p| {
            adj_points(p, grid.len(), grid[0].len()).filter(move |&(r, c)| {
                grid[p.0][p.1] + 1 >= grid[r][c]
            })
            .map(move |ap| (p, ap))
        })
        .collect::<HashSet<_>>()
}

fn one(graph: &FastGraph, w: usize, start: &Coord, finish: &Coord) -> usize {
    if let Some(shortest_path) = fast_paths::calc_path(graph, start.0 * w + start.1, finish.0 * w + finish.1) {
        if shortest_path.is_found() {
            return shortest_path.get_weight()
        }
    }
    0
}

fn starting_points(grid: &[&[i32]]) -> Vec<Coord>{
    let h = grid.len();
    let w = grid[0].len();
    let mut res = Vec::with_capacity(400);
    for row in 1..h - 1 {
        if grid[row][0] == ('a' as i32) {
            res.push((row, 0));
        }
        if grid[row][w - 1] == ('a' as i32) {
            res.push((row, w - 1));
        }
    }
    for col in 0..w {
        if grid[0][col] == ('a' as i32) {
            res.push((0, col));
        }
        if grid[h - 1][col] == ('a' as i32) {
            res.push((h - 1, col));
        }
    }
    res
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut buf: Vec<i32> = Vec::with_capacity(10000);
    let (grid_vec, start, finish) = read_to_array(it, &mut buf);
    let grid = grid_vec.as_slice();
    let w = grid[0].len();
    let mut input_graph = InputGraph::new();
    adj_map(grid).iter()
        .map(|&(from, to)| (from.0 * w + from.1, to.0 * w + to.1))
        .for_each(|(from, to)| { input_graph.add_edge(from, to, 1); });
    input_graph.freeze();
    let fast_graph = fast_paths::prepare(&input_graph);
    (if part_two {
        starting_points(grid).iter()
            .map(|s| one(&fast_graph, w, s, &finish))
            .filter(|weight| *weight != 0)
            .min()
            .unwrap_or_default()
    } else {
        one(&fast_graph, w, &start, &finish)
    }).to_string().into()
}

#[test]
fn test1() {
    assert_eq!("31", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
}

#[test]
fn test2() {
    assert_eq!("29", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true));
}
