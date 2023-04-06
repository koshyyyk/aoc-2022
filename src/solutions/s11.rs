use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use itertools::{Itertools, izip};

use crate::{InputIterator, Ztr};

const PRIMES: [u32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

struct Monkey {
    items: RefCell<Vec<Vec<u32>>>,
    op: fn(&[u32], &[u32]) -> Vec<u32>,
    op_param: Vec<u32>,
    test_param: u32,
    dst: (usize, usize),
    count: usize
}

impl Monkey {

    fn new(items: &[u32], op: fn(&[u32], &[u32]) -> Vec<u32>, op_param: u32, test_param: u32, dst: (usize, usize)) -> Self {
        Monkey {
            items: RefCell::new(items.iter().map(|&n| to_rns(n)).collect()),
            op,
            op_param: to_rns(op_param),
            test_param,
            dst,
            count: 0
        }
    }

    fn examine(&mut self) -> Vec<(usize, Vec<u32>)> {
        self.count += self.items.borrow().len();
        let r = self.items.borrow().iter()
            .map(|item| {
                let wl = (self.op)(item, &self.op_param);
                let dst = if rns_test(&wl, self.test_param) {
                    self.dst.0
                } else {
                    self.dst.1
                };
                (dst, wl)
            })
            .collect::<Vec<_>>();
        self.items.borrow_mut().clear();
        r
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey items: {:?}, count: {}", self.items.borrow(), self.count)
    }
}

fn to_rns(i: u32) -> Vec<u32> {
    PRIMES.iter().map(|&p| i % p).collect()
}

fn rns_square(l: &[u32], _: &[u32]) -> Vec<u32> {
    l.iter().enumerate().map(|(n, &k)| (k * k) % PRIMES[n]).collect()
}

fn rns_mul(l: &[u32], r: &[u32]) -> Vec<u32> {
    izip!(l, r).enumerate().map(|(n, (&ll, &rr))| (ll * rr) % PRIMES[n]).collect()
}

fn rns_add(l: &[u32], r: &[u32]) -> Vec<u32> {
    izip!(l, r).enumerate().map(|(n, (&ll, &rr))| (ll + rr) % PRIMES[n]).collect()
}

fn rns_sub(l: &[u32], r: &[u32]) -> Vec<u32> {
    izip!(l, r).enumerate().map(|(n, (&ll, &rr))| (PRIMES[n] - rr + ll) % PRIMES[n]).collect()
}

fn rns_test(n: &[u32], d: u32) -> bool {
    PRIMES.iter().enumerate().any(|(i, &p)| d == p && n[i] == 0)
}

fn round(monkeys: &mut [Monkey], _: u32) {
    for n in 0..monkeys.len() {
        let m = &mut monkeys[n];
        m.examine().into_iter().for_each(|(dst, item)| {
            //println!("monkey {} throws {} to {}", n, item, dst);
            monkeys[dst].items.borrow_mut().push(item);
        });
    }
}


pub fn solution(_: InputIterator, part_two: bool) -> Ztr {
    let mut monkeys = vec![
        Monkey::new(&vec![76, 88, 96, 97, 58, 61, 67], rns_mul, 19, 3, (2, 3)),
        Monkey::new(&vec![93, 71, 79, 83, 69, 70, 94, 98], rns_add, 8, 11, (5, 6)),
        Monkey::new(&vec![50, 74, 67, 92, 61, 76], rns_mul, 13, 19, (3, 1)),
        Monkey::new(&vec![76, 92], rns_add, 6, 5, (1, 6)),
        Monkey::new(&vec![74, 94, 55, 87, 62], rns_add, 5, 2, (2, 0)),
        Monkey::new(&vec![59, 62, 53, 62], rns_square, 0, 7, (4, 7)),
        Monkey::new(&vec![62], rns_add, 2, 17, (5, 7)),
        Monkey::new(&vec![85, 54, 53], rns_add, 3, 13, (4, 0)),
    ];
    let (c, d) = if part_two {
        (10000, 1)
    } else {
        (20, 3)
    };
    for _ in 0..c {
        round(&mut monkeys, d);
    }
    let r: usize = monkeys.iter().map(|m| m.count).sorted().skip(6).product();
    r.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let r = rns_mul(&to_rns(8), &to_rns(10));
        assert_eq!(to_rns(80), r);
        let ten = to_rns(10);
        let eight = to_rns(8);
        let rr = rns_add(&eight, &ten);
        assert_eq!(to_rns(18), rr);
        assert!(rns_test(&rr, 3));
        let rrr = rns_sub(&ten, &eight);
        assert_eq!(to_rns(2), rrr);
    }

    #[test]
    fn test1() {
        let mut monkeys = vec![
                Monkey::new(&vec![79, 98], rns_mul, 19, 23, (2, 3)),
                Monkey::new(&vec![54, 65, 75, 74], rns_add, 6, 19, (2, 0)),
                Monkey::new(&vec![79, 60, 97], rns_square, 0, 13, (1, 3)),
                Monkey::new(&vec![74], rns_add, 3, 17, (0, 1)),
            ];
        for _ in 0..20 {
            round(&mut monkeys, 3);
            //println!("0: {}\n1: {}\n2: {}\n3: {}", monkeys[0], monkeys[1], monkeys[2], monkeys[3]);
        }
        let r: usize = monkeys.iter().map(|m| m.count).sorted().skip(2).product();
        assert_eq!(10605, r);
    }

    #[test]
    fn test2() {
        let mut monkeys = vec![
            Monkey::new(&vec![79, 98], rns_mul, 19, 23, (2, 3)),
            Monkey::new(&vec![54, 65, 75, 74], rns_add, 6, 19, (2, 0)),
            Monkey::new(&vec![79, 60, 97], rns_square, 0, 13, (1, 3)),
            Monkey::new(&vec![74], rns_add, 3, 17, (0, 1)),
        ];
        for _ in 0..20 {
            round(&mut monkeys, 1);
        }
        //println!("0: {}\n1: {}\n2: {}\n3: {}", monkeys[0], monkeys[1], monkeys[2], monkeys[3]);
        let mut r: usize = monkeys.iter().map(|m| m.count).sorted().skip(2).product();
        assert_eq!(103 * 99, r);
        for _ in 20..2000 {
            round(&mut monkeys, 1);
        }
        r = monkeys.iter().map(|m| m.count).sorted().skip(2).product();
        assert_eq!(10419 * 10391, r);
        for _ in 2000..10000 {
            round(&mut monkeys, 1);
        }
        r = monkeys.iter().map(|m| m.count).sorted().skip(2).product();
        assert_eq!(2713310158, r);
    }
}
