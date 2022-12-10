use pom::parser::{Parser, sym};

use crate::{InputIterator, Ztr};
use crate::parsers::integer;

static TEST_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn range<'a>() -> Parser<'a, u8, (i32, i32)> {
    (integer() - sym(b'-').discard()) + integer()
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let parser = range() - sym(b',').discard() + range();
    parser.parse(line.as_bytes()).unwrap_or_default()
}

#[test]
fn real_input_parses() {
    assert_eq!(((85, 97), (86, 99)), parse_line("85-97,86-99"));
}

fn is_within(&((fs, fe), (ss, se)): &((i32, i32), (i32, i32))) -> bool {
    (fs <= ss && fe >= se) || (ss <= fs && se >= fe)
}

#[test]
fn complete_overlap_works() {
    assert!(is_within(&((2, 8), (3, 7))));
    assert!(is_within(&((6, 6), (4, 6))));
    assert!(!is_within(&((2, 4), (6, 8))));
}

fn is_overlap(&((fs, fe), (ss, se)): &((i32, i32), (i32, i32))) -> bool {
    (fe >= ss && fe <= se) || (se >= fs && se <= fe)
}

fn f1 (i: InputIterator) -> usize {
    i.map(|l| parse_line(&l)).filter(is_within).count()
}

fn f2(i: InputIterator) -> usize {
    i.map(|l| parse_line(l.as_ref())).filter(is_overlap).count()
}

pub fn solution(i: InputIterator, part_two: bool) -> Ztr {
    (if part_two {
        f2(i)
    } else {
        todo!()
    }).to_string().into()
}

#[test]
fn test1() {
    assert_eq!(2, f1(& mut (TEST_DATA.lines().map(|s| s.into()))));
}

#[test]
fn test2() {
    assert_eq!(4, f2(& mut (TEST_DATA.lines().map(|s| s.into()))));
}
