use std::collections::VecDeque;

use itertools::Itertools;

use crate::{InputIterator, Ztr};

static _TEST_DATA: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

fn find_marker(i: InputIterator, len: usize) -> usize {
    let mut buf: VecDeque<char> = VecDeque::new();
    i.next()
     .iter()
     .flat_map(|line| line.chars()
                         .enumerate()
                         .find(|&(_, c)| {
                             if buf.iter().unique().count() == len {
                                 true
                             } else {
                                 if buf.len() == len {
                                     buf.pop_front();
                                 }
                                 buf.push_back(c);
                                 false
                             }
                         })
                         .map(|(i, _)| i))
     .last()
     .unwrap_or_default()
}

pub fn solution(i: InputIterator, part_two: bool) -> Ztr {
    let n = if part_two { 4 } else { 14 };
    find_marker(i, n).to_string().into()
}

#[test]
fn test1() {
    assert_eq!(11, find_marker(& mut (_TEST_DATA.lines().map(|s| s.into())), 4));
}

#[test]
fn test2() {
    assert_eq!(26, find_marker(& mut (_TEST_DATA.lines().map(|s| s.into())), 14));
}
