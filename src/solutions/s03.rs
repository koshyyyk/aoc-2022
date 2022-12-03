use itertools::{Itertools, izip};

use crate::{InputIterator, Ztr};

static TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn prio(c: char) -> i32 {
    (if c.is_lowercase() {
        c as i32 - 'a' as i32
    } else if c.is_uppercase() {
        c as i32 - 'A' as i32 + 26
    } else {
        -1
    }) + 1
}

#[test]
fn test_prio() {
    assert_eq!(16, prio('p'));
    assert_eq!(42, prio('P'));
}

fn find_same_item(parts: &[&str]) -> Option<char> {
    parts[0].chars().find(|c| parts[1..].iter().map(|ztr| ztr.chars().contains(c)).all(|s| s))
}

fn prio_for_line(line: &str) -> i32 {
    let middle = line.len() / 2;
    find_same_item(&[&line[..middle], &line[middle..]]).map(prio).unwrap_or_default()
}

#[test]
fn some_prio() {
    assert_eq!(16, prio_for_line("vJrwpWtwJgWrhcsFMMfFFhFp"));
    assert_eq!(38, prio_for_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
}

fn prio_for_group(group: &[&str]) -> i32 {
    find_same_item(group).map(prio).unwrap_or_default()
}

#[test]
fn some_prio_for_group() {
    let group1 = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg"
    ];
    assert_eq!(18, prio_for_group(&group1));
    let group2 = [
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw"
    ];
    assert_eq!(52, prio_for_group(&group2));
}

fn f1(i: InputIterator) -> i32 {
    i.map(|l| prio_for_line(&l)).sum()
}

fn f2(i: InputIterator) -> i32 {
    let lines = i.collect::<Vec<_>>();
    izip!(lines.iter().step_by(3), lines[1..].iter().step_by(3), lines[2..].iter().step_by(3))
        .map(|(l1, l2, l3)| prio_for_group(&[l1, l2, l3]))
        .sum()
}

pub fn solution(i: InputIterator) -> (Ztr, Ztr) {
    (f2(i).to_string().into(), "--".into())
}

#[test]
fn test1() {
    assert_eq!(157, f1(& mut (TEST_DATA.lines().map(|s| s.into()))));
}

#[test]
fn test2() {
    assert_eq!(70, f2(& mut (TEST_DATA.lines().map(|s| s.into()))));
}
