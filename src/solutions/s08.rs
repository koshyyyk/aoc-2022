use std::collections::HashSet;

use itertools::{Either, Itertools};

use crate::{InputIterator, Ztr};

static _TEST_DATA: &str = "30373
25512
65332
33549
35390
";

fn read_to_array<'a>(it: InputIterator, buf: &'a mut Vec<i32>) -> Vec<&'a [i32]> {
    let mut width = 0_usize;
    it.for_each(|ztr| {
        width = ztr.len();
        buf.append(& mut ztr.chars().map(|c| c as i32).collect::<Vec<_>>());
    });
    buf.as_slice().chunks(width).collect()
}

fn visibility_check(istart: usize, iend: usize, ostart: usize, oend: usize, transp: bool, patch: &[&[i32]], visible: &mut HashSet<(usize, usize)>) {
    let oiter = if ostart < oend {
        Either::Left(ostart..=oend)
    } else {
        Either::Right((oend..=ostart).rev())
    };
    oiter.for_each(|oi| {
        let iiter = if istart < iend {
            Either::Left(istart..=iend)
        } else {
            Either::Right((iend..=istart).rev())
        };
        let mut current_height = -1;
        iiter.for_each(|ii| {
            let (ri, ci) = if transp { (ii, oi) } else { (oi, ii) };
            let height = patch[ri][ci];
            if height > current_height {
                visible.insert((ri, ci));
                current_height = height;
            }
        })
    })
}

fn score_helper<I, F>(it: &mut I, current: i32, lens: F) -> i32
    where F: Fn(usize) -> i32,
          I: Iterator<Item = usize>
{
    let mut aac: i32 = 0;
    it.try_fold(0, |acc, c| {
        match lens(c) {
            v if v < current => {aac = acc + 1; Some(aac)},
            _ => {aac = acc + 1; None}
        }
    });
    aac
}

fn score(row: usize, col: usize, patch: &[&[i32]]) -> i32 {
    let h = patch.len();
    let w = patch[0].len();
    let current_height = patch[row][col];
    let right= score_helper(&mut (col + 1..w), current_height, |c| patch[row][c]);
    let left = score_helper(&mut (0..col).rev(), current_height, |c| patch[row][c]);
    let down = score_helper(&mut (row + 1..h), current_height, |r| patch[r][col]);
    let up = score_helper(&mut (0..row).rev(), current_height, |r| patch[r][col]);
    let partial_scores = vec![up, left, down, right];
    partial_scores.iter().product()
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut buf: Vec<i32> = Vec::with_capacity(10000);
    let patch_vec = read_to_array(it, &mut buf);
    let patch = patch_vec.as_slice();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let w = patch[0].len();
    let h = patch.len();
    let res = if part_two {
        (0..h).cartesian_product(0..w)
            .map(|(r, c)| score(r, c, patch))
            .max()
            .unwrap_or_default()
            .to_string()
    } else {
        let hr = w - 1;
        let vr = h - 1;
        visibility_check(0, hr, 0, vr, false, patch, &mut visible);
        visibility_check(0, hr, 0, vr, true, patch, &mut visible);
        visibility_check(hr, 0, vr, 0, true, patch, &mut visible);
        visibility_check(hr, 0, vr, 0, false, patch, &mut visible);
        visible.len().to_string()
    };
    res.into()
}

#[test]
fn test1() {
    let mut buf: Vec<i32> = Vec::with_capacity(1000);
    let rr = read_to_array(& mut (_TEST_DATA.lines().map(|s| s.into())), &mut buf);
    let patch = rr.as_slice();
    assert_eq!('3' as i32, patch[0][0]);
    assert_eq!('0' as i32, patch[4][4]);
    assert_eq!(5, patch[0].len());
    assert!(patch[0][1] < patch[0][2]);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let hr = patch[0].len() - 1;
    let vr = patch.len() - 1;
    visibility_check(0, hr, 0, vr, false, patch, &mut visible);
    visibility_check(0, hr, 0, vr, true, patch, &mut visible);
    visibility_check(hr, 0, vr, 0, true, patch, &mut visible);
    visibility_check(hr, 0, vr, 0, false, patch, &mut visible);
    //println!("{:?}", &visible);
    assert_eq!(21, visible.len());
    let w = patch[0].len();
    let h = patch.len();
    assert_eq!(2, score_helper(&mut (0..3).rev(), '5' as i32, |r| patch[r][2]));
    assert_eq!(1, score_helper(&mut (4..h), '5' as i32, |r| patch[r][2]));
    assert_eq!(2, score_helper(&mut (0..2).rev(), '5' as i32, |c| patch[3][c]));
    assert_eq!(2, score_helper(&mut (3..w), '5' as i32, |c| patch[3][c]));

    assert_eq!(0, score(0, 0, patch));
    assert_eq!(8, score(3, 2, patch));
}

#[test]
fn test2() {
    assert!(true);
}
