use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex::{self};

use crate::{InputIterator, Ztr};

static TEST_DATA: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

fn update_sizes(path: &Vec<String>, sizes: &mut HashMap<String, i32>, size: i32) {
    let mut index = 0;
    while index <= path.len() {
        let path_str = path.iter().take(index).join("/");
        *sizes.entry(path_str).or_insert(0) += size;
        index += 1;
    }
}

fn modify_path(path: & mut Vec<String>, c: &str, line: usize) {
    match c {
        "/" => {
            path.clear();
        },
        ".." => {
            if path.pop().is_none() {
                panic!("error traversing. line {}", line)
            }
        },
        p => path.push(p.to_string())
    }
}

fn walk_os_tree(i: InputIterator) -> i32 {
    let cd = Regex::new(r"cd\s+(?P<name>\S+)").unwrap();
    let file = Regex::new(r"^(?P<size>\d+)\s+\S+").unwrap();
    let mut path: Vec<String> = vec![];
    let mut sizes: HashMap<String, i32> = HashMap::new();
    let mut size = 0_i32;
    for (line, str) in i.enumerate() {
        cd.captures(str.as_ref()).iter().for_each(|c| {
            update_sizes(&path, &mut sizes, size);
            size = 0;
            modify_path(& mut path, &c["name"], line);
        });
        file.captures(str.as_ref()).iter().for_each(|c| {
            size += i32::from_str(&c["size"]).unwrap_or_default();
        });
        if str.starts_with("$ ls") {
            size = 0;
        }
    }
    update_sizes(&path, &mut sizes, size);
    let total_size = &sizes.get("").map(|v| *v).unwrap_or_default();
    let to_free = 30000000 - (70000000 - total_size);
    println!("total size: {}, to free: {}", total_size, to_free);
    //sizes.iter().map(|e| *e.1).filter(|s| *s < 100000).sum()
    sizes.iter().map(|e| *e.1).sorted().skip_while(|s| *s < to_free).next().unwrap_or_default()
}

pub fn solution(i: InputIterator, part_two: bool) -> Ztr {
    (if part_two {
        walk_os_tree(i)
    } else {
        todo!()
    }).to_string().into()

}

#[test]
fn test1() {
    let cd = Regex::new(r"cd\s+(\W+)");
    let captures = cd.iter().flat_map(|re| re.captures_iter("jj")).collect::<Vec<_>>();
    println!("{:?}", captures);
}

#[test]
fn test2() {
    assert_eq!(95437, walk_os_tree(& mut (TEST_DATA.lines().map(|s| s.into()))));
}

#[test]
fn test3() {
    assert_eq!(24933642, walk_os_tree(& mut (TEST_DATA.lines().map(|s| s.into()))));
}
