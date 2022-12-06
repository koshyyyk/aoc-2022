use pom::parser::{seq};

use crate::{InputIterator, Ztr};
use crate::parsers::{integer, space};

static TEST_DATA: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

fn parse_move(line: &str) -> (i32, i32, i32) {
    let parser = (seq(b"move").discard() + space()) * integer() +
                 (space() + seq(b"from").discard() + space()) * integer() +
                 (space() + seq(b"to").discard() + space()) * integer();
    let parsed = parser.parse(line.as_bytes()).unwrap_or_default();
    (parsed.0.0, parsed.0.1, parsed.1)
}

#[test]
fn move_parsed_correctly() {
    assert_eq!((2, 2, 7), parse_move("move 2 from 2 to 7"));
}

fn parse_crates(line: &str) -> Vec<(usize, char)> {
    line.chars()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .collect::<Vec<_>>()
}

fn add_to_stacks(stacks: & mut Vec<Vec<char>>, krates: &str) {
    parse_crates(krates).iter().for_each(|&(i, item)| {
        while stacks.len() <= i {
            stacks.push(Vec::with_capacity(16))
        }
        stacks[i].insert(0, item);
    })
}

fn parse(i: InputIterator) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(16);
    let mut instructions: Vec<(i32, i32, i32)> = Vec::new();
    for line in i {
        match &line {
            krates if krates.contains("[") => add_to_stacks(& mut stacks, krates),
            instr if line.contains("move") => instructions.push(parse_move(instr)),
            _ => ()
        }
    };
    (stacks, instructions)
}

#[test]
fn partial_read() {
    let (mut stacks, mut instructions) = parse(& mut (TEST_DATA.lines().map(|s| s.into())));
    assert_eq!(Some('N'), stacks[0].pop());
    assert_eq!(Some('D'), stacks[1].pop());
    assert_eq!(Some((1, 1, 2)), instructions.pop());
}

fn move_9001(stacks: & mut Vec<Vec<char>>, (num, from, to): (i32, i32, i32)) {
    let offset = stacks[from as usize].len() - num as usize;
    let mut krates = Vec::from(&stacks[from as usize][offset..]);
    stacks[from as usize].truncate(offset);
    stacks[to as usize].append(& mut krates);
}

fn move_9000(stacks: & mut Vec<Vec<char>>, (mut num, from, to): (i32, i32, i32)) {
    while num > 0 {
        let krate = stacks[from as usize].pop().unwrap_or_default();
        stacks[to as usize].push(krate);
        num -= 1;
    }
}

fn f1(i: InputIterator) -> String {
    let (mut stacks, instructions) = parse(i);
    instructions.iter()
                .for_each(|&(num, from, to)| move_9001(& mut stacks, (num, from - 1, to - 1)));
    stacks.iter()
          .map(|stack| stack.last())
          .flat_map(|opt| opt)
          .collect()
}

#[test]
fn test_move() {
    let (mut stacks, instructions) = parse(& mut (TEST_DATA.lines().map(|s| s.into())));
    instructions.iter().for_each(|&(num, from, to)| move_9001(& mut stacks, (num, from - 1, to - 1)));
    assert_eq!(Some('M'), stacks[0].pop());
    assert_eq!(Some('C'), stacks[1].pop());
    assert_eq!(Some('D'), stacks[2].pop());
}

pub fn solution(i: InputIterator) -> (Ztr, Ztr) {
    (f1(i).into(), "--".into())
}
