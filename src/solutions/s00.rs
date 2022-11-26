use itertools::Itertools;
use crate::{InputIterator};

pub fn solution(i: InputIterator) -> String {
    format!("Testing plugin system. Parameters: {}", i.join(" "))
}

#[test]
fn test() {
    assert!(true);
}
