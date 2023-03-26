use std::collections::HashMap;
use std::str::{from_utf8, FromStr};

use regex::bytes::Regex::{self};

use Monkey::*;

use crate::{InputIterator, Ztr};

type Name = u32;

type Mm = HashMap<Name, Monkey>;

#[derive(Debug, Copy, Clone)]
enum Monkey {
    YELL(Name, Option<i64>),
    WAIT(Name, Name, Name, char),
    ROOT(Name, Name, Name),
}

fn eval(mm: &Mm, name: &Name) -> Option<i64> {
    match mm.get(name) {
        Some(&YELL(_, n)) => n,
        Some(WAIT(_, l, r, op)) => {
            if let Some(op1) = eval(mm, l) {
                if let Some(op2) = eval(mm, r) {
                    return Some(match *op {
                        '+' => op1 + op2,
                        '-' => op1 - op2,
                        '*' => op1 * op2,
                        '/' => op1 / op2,
                        _ => panic!("unsupported op: {}", *op)
                    })
                }
            }
            None
        }
        Some(ROOT(_, l, r)) => {
            match (eval(mm, l), eval(mm, r)) {
                (None, Some(n)) => Some(solve(mm, l, n)),
                (Some(n), None) => Some(solve(mm, r, n)),
                _ => panic!("shouldn't happen")
            }
        }
        _ => panic!("No def for monkey named {:?}", name)
    }
}

fn solve(mm: &Mm, name: &Name, i: i64) -> i64 {
    match mm.get(name) {
        Some(&YELL(_, None)) => i, // THE answer
        Some(&YELL(_, Some(n))) => n,
        Some(&WAIT(_, l, r, op)) => {
            match (op, eval(mm, &l), eval(mm, &r)) {
                ('+', None, Some(n)) => solve(mm, &l, i - n),
                ('+', Some(n), None) => solve(mm, &r, i - n),
                ('-', None, Some(n)) => solve(mm, &l, i + n),
                ('-', Some(n), None) => solve(mm, &r, n - i),
                ('*', None, Some(n)) => solve(mm, &l, i / n),
                ('*', Some(n), None) => solve(mm, &r, i / n),
                ('/', None, Some(n)) => solve(mm, &l, i * n),
                ('/', Some(n), None) => solve(mm, &r, n / i),
                (op, Some(op1), Some(op2)) => {
                    match op {
                        '+' => op1 + op2,
                        '-' => op1 - op2,
                        '*' => op1 * op2,
                        '/' => op1 / op2,
                        _ => panic!("unsupported op: {}", op)
                    }
                }
                _ => panic!("nooo")
            }
        },
        _ => panic!("No def for monkey named {:?}", name)
    }
}

fn to_name(n: &[u8]) -> u32 {
    (n[0] as u32) << 24 | (n[1] as u32) << 16 | (n[2] as u32) << 8 | n[3] as u32
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut mm: Mm = HashMap::with_capacity(1000);
    let wait = Regex::new(r"(\w{4}): (\w{4}) ([+-/*]) (\w{4})").unwrap();
    let yell = Regex::new(r"(\w{4}): (\d+)").unwrap();
    let root = to_name(b"root");
    let humn = to_name(b"humn");
    for line in it {
        let l = line.as_bytes();
        wait.captures(l).into_iter().for_each(|c| {
            let n = to_name(&c[1]);
            if part_two && n == root {
                mm.insert(n, ROOT(n, to_name(&c[2]), to_name(&c[4])));
            } else {
                mm.insert(n, WAIT(n, to_name(&c[2]), to_name(&c[4]), c[3][0] as char));
            }
        });
        yell.captures(l).into_iter().for_each(|c| {
            let n = to_name(&c[1]);
            if part_two && n == humn {
                mm.insert(n, YELL(n, None));
            } else {
                let val = i64::from_str(from_utf8(&c[2]).unwrap()).unwrap_or_default();
                mm.insert(n, YELL(n, Some(val)));
            }
        });
    }
    eval(&mm, &root).unwrap_or_default().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_DATA: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test1() {
        assert_eq!("152", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

    #[test]
    fn test2() {
        assert_eq!("301", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true));
    }

}
