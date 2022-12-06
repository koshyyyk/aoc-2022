use std::cmp::Reverse;

use crate::{InputIterator, Ztr};

static TEST_DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

fn helper(i: InputIterator) -> Box<Vec<usize>> {
    let mut acc = Box::new(Vec::with_capacity(16));
    acc.push(0);
    for r in i.map(|l| l.parse::<usize>()) {
        match r {
            Ok(num) => {
                if let Some(last) = acc.pop() {
                    acc.push(last + num)
                }
            }
            _ => acc.push(0)
        }
    }
    acc.sort_by_key(|w| Reverse(*w));
    acc
}

pub fn solution(i: InputIterator) -> (Ztr, Ztr) {
    let sorted = helper(i);
    (sorted[0..1].iter().sum::<usize>().to_string().into(),
     sorted[0..3].iter().sum::<usize>().to_string().into())
}

#[test]
fn test1() {
    assert_eq!(solution(&mut (TEST_DATA.lines().map(|s| s.into()))).0, "24000");
}

#[test]
fn test2() {
    assert_eq!(solution(&mut (TEST_DATA.lines().map(|s| s.into()))).1, "45000");
}