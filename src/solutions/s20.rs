use std::str::FromStr;

use skiplist::SkipList;

use crate::{InputIterator, Ztr};

#[derive(Debug, Default, Copy, Clone)]
struct Item {
    val: i64,
    num: usize,
    round: i32
}

fn next(cur: i64, shift: i64, size: i64) -> (usize, usize) {
    let mut n = (cur + shift)  % (size - 1);
    n = if n == 0 && shift > 0 {
        size - 1
    } else if n < 0 {
        n + size - 1
    } else {
        n
    };
    (n as usize, if n > cur { 0 } else { 1 } as usize)
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let key = if part_two { 811589153 } else { 1 };
    let rounds = if part_two { 10 } else { 1 };
    let mut size = 0;
    let mut numbers: SkipList<Item> = it
        .filter_map(|line| i64::from_str(&line).ok())
        .inspect(|_| { size += 1; })
        .enumerate()
        .map(|(num, val)| Item { val: val * key, round: 0, num })
        .collect();
    println!("size {}", size);
    let mut current = 0;
    for current_round in 1..=rounds {
        println!("round {}", current_round);
        let mut to_handle = size;
        let mut item_to_handle = 0_usize;
        while to_handle > 0 {
            let item @ Item { val, round, num} = numbers[current];
            if round < current_round && item_to_handle == num {
                to_handle -= 1;
                item_to_handle += 1;
                if val != 0 {
                    numbers.remove(current);
                    let nxt = next(current as i64, val, size as i64);
                    current += nxt.1;
                    //println!("{} got {}", val, index);
                    numbers.insert(Item { round: current_round, ..item }, nxt.0);
                } else {
                    numbers[current].round = current_round;
                    current += 1;
                }
            } else {
                current += 1;
            }
            current = current % size;
        }
    }
    //println!("{:?}", numbers.iter().map(|i| i.val).collect::<Vec<_>>());
    let zpos = numbers.iter().position(|i| i.val == 0).unwrap_or_default();
    let r: i64 = vec![1000, 2000, 3000].into_iter().map(|n| numbers[(zpos + n) % size].val).sum();
    r.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_DATA: &str = "1
2
-3
3
-2
0
4
";
    #[test]
    fn test0() {
        assert_eq!((3, 0), next(2, 1, 5));
        assert_eq!((4, 0), next(2, 2, 5));
        assert_eq!((2, 1), next(2, 4, 5));
        assert_eq!((1, 1), next(2, 7, 5));
        assert_eq!((0, 1), next(2, -2, 5));
        assert_eq!((1, 1), next(2, -5, 5));
    }

    #[test]
    fn test1() {
        assert_eq!("3", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

    #[test]
    fn test2() {
        assert_eq!("1623178306", solution(& mut (_TEST_DATA.lines().map(|s| s.into())), true));
    }
}
