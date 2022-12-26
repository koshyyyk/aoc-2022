use std::collections::HashSet;
use std::str::FromStr;

use crate::{InputIterator, Ztr};

fn num_neighbours(&(x, y, z): &(i32, i32, i32), world: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut n = 0;
    if world.contains(&(x - 1, y, z)) {
        n += 1;
    }
    if world.contains(&(x + 1, y, z)) {
        n += 1;
    }
    if world.contains(&(x, y - 1, z)) {
        n += 1;
    }
    if world.contains(&(x, y + 1, z)) {
        n += 1;
    }
    if world.contains(&(x, y, z - 1)) {
        n += 1;
    }
    if world.contains(&(x, y, z + 1)) {
        n += 1;
    }
    n
}

pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let mut sides: i32 = 0;
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    it.map(|line| {
            let mut coords = line.split(",").map(|coord| i32::from_str(coord).unwrap_or_default());
            (coords.next().unwrap_or_default(), coords.next().unwrap_or_default(), coords.next().unwrap_or_default())
        })
        .for_each(|cube| {
            sides += 6 - num_neighbours(&cube, &cubes) * 2;
            cubes.insert(cube);
        });
    (if part_two {
        let xmax = cubes.iter().map(|&(x, _, _)| x).max().unwrap_or_default();
        let ymax = cubes.iter().map(|&(_, y, _)| y).max().unwrap_or_default();
        let zmax = cubes.iter().map(|&(_, _, z)| z).max().unwrap_or_default();
        for z in 1..=zmax {
            for y in 1..=ymax {
                for x in 1..=xmax {
                    let cube = (x, y, z);
                    if !cubes.contains(&cube) {
                        sides -= match num_neighbours(&cube, &cubes) {
                            n @ 3..=6 => n,
                            _ => 0
                        }
                    }
                }
            }
        }
        sides
    } else {
        sides
    }).to_string().into()
}

#[cfg(test)]
mod test {
    use super::*;

    static _TEST_DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test1() {
        assert_eq!("64", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

    #[test]
    fn test2() {
        assert_eq!("58", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), true));
    }
}
