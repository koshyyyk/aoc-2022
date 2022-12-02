use crate::{InputIterator, Ztr};

static TEST_DATA: &str = "A Y
B X
C Z";

fn rps(round: &[u32]) -> u32 {
    match round {
        [opp, you] if opp == you => you + 3,
        [1, 2] => 6 + 2,
        [1, 3] => 0 + 3,
        [2, 1] => 0 + 1,
        [2, 3] => 6 + 3,
        [3, 1] => 6 + 1,
        [3, 2] => 0 + 2,
        _ => 0
    }
}

fn rps_match_outcome(round: &[u32]) -> u32 {
    match round {
        [opp, 1] => 0 + if *opp == 1 { 3 } else { opp - 1 },
        [opp, 2] => 3 + opp,
        [opp, 3] => 6  + if *opp == 3 { 1 } else { opp + 1 },
        _ => 0
    }
}

fn to_processed(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_digit(36))
        .map(|n| if n > 30 { n - 32 } else { n - 9 })
        .collect()
}


fn score_for_line(line: &str) -> u32 {
   rps(&to_processed(line))
}

fn move_for_line(line: &str) -> u32 {
    rps_match_outcome(&to_processed(line))
}

fn combine(i: InputIterator, f: fn(&str) -> u32) -> u32 {
    i.map(|l| f(&l)).fold(0, |s, v| s + v)
}

fn p1(i: InputIterator) -> u32 {
    combine(i, score_for_line)
}


pub fn solution(i: InputIterator) -> (Ztr, Ztr) {
    ("--".into(),
     combine(i, move_for_line).to_string().into())
}

#[test]
fn linescore() {
    assert_eq!(8, score_for_line("A Y"));
    assert_eq!(1, score_for_line("B X"));
    assert_eq!(6, score_for_line("C Z"));
}

#[test]
fn test1() {
    assert_eq!(15, p1(& mut (TEST_DATA.lines().map(|l| l.into()))));
}

#[test]
fn match_score() {
    assert_eq!(4, move_for_line("A Y"));
    assert_eq!(1, move_for_line("B X"));
    assert_eq!(7, move_for_line("C Z"));
}

#[test]
fn test2() {
    assert_eq!(12, combine(& mut (TEST_DATA.lines().map(|l| l.into())), move_for_line));
}
