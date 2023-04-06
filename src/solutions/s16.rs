use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use fast_paths::{FastGraph, InputGraph};
use pom::parser::{is_a, list, Parser, seq};

use crate::{InputIterator, Ztr};
use crate::parsers::integer;

const TIME: usize = 30;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Valve(char, char);

impl Valve {
    fn from_arr(v: &[u8]) -> Self {
        Valve(v[0] as char, v[1] as char)
    }

    fn node_id(&self) -> usize {
        1 + (self.0 as usize - 65) << 8 | (self.1 as usize - 65)
    }

}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

fn valve<'a>() -> Parser<'a, u8, Valve> {
    is_a(|s| s > b'@' && s < b'[').repeat(2).collect().map(|m| Valve::from_arr(m))
}

fn parser<'a>() -> Parser<'a, u8, (Valve, i32, Vec<Valve>)> {
    let singleton_valve = (seq(b"; tunnel leads to valve ") * valve()).map(|v| vec![v]);
    let valves = seq(b"; tunnels lead to valves ") * list(valve(), seq(b", "));
    (seq(b"Valve ") * valve() + seq(b" has flow rate=") * integer() + (singleton_valve | valves))
        .map(|((src, rate), dst)| (src, rate, dst))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    valve: Valve,
    rate: i32,
    instant: usize,
    max_flow: i32,
    visited: bool,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} flow {}", self.valve, self.max_flow)
    }
}

struct World {
    nodes: Vec<Node>,
    valve_graph: FastGraph,
    distance_map: RefCell<HashMap<(usize, usize), usize>>
}

impl World {
    fn create(it: InputIterator) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        // let mut rates = HashMap::new();
        let mut input_graph = InputGraph::new();
        for (valve, rate, connections) in it.flat_map(|line| parser().parse(line.as_bytes()).into_iter()) {
            if rate > 0 {
                nodes.push(Node { valve, rate, instant: 0, visited: false, max_flow: 0 });
            }
            for connection in connections.iter() {
                input_graph.add_edge(valve.node_id(), connection.node_id(), 1);
            }
        }
        input_graph.freeze();
        //nodes.push(Node { valve: Valve('A', 'A'), rate: 0, instant: 0, max_flow: 0, visited: false });
        World{ nodes, valve_graph: fast_paths::prepare(&input_graph), distance_map: RefCell::new(HashMap::new()) }
    }

    fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    fn distance(&self, from: &Valve, to: &Valve) -> Option<usize> {
        if *from == *to {
            return Some(0);
        }
        let from_id = from.node_id();
        let to_id = to.node_id();
        if let Some(d) = self.distance_map.borrow().get(&(from_id, to_id)) {
            return Some(*d);
        }
        if let Some(d) = self.distance_map.borrow().get(&(to_id, from_id)) {
            return Some(*d);
        }
        if let Some(path) = fast_paths::calc_path(&self.valve_graph, from_id, to_id) {
            if path.is_found() {
                let distance = path.get_weight();
                self.distance_map.borrow_mut().insert((from_id, to_id), distance);
                println!("distance from {} to {} set to {}", from, to, distance);
                return Some(distance);
            }
        }
        None
    }

    fn weight(&self, current: &Valve, target: &Valve, _: usize) -> Option<usize> {
        self.distance(current, target).and_then(|distance| Some(distance + 1))
    }

    fn val(&self, instant: usize, current: &Node, target: &Node) -> Option<(usize, i32)> {
        self.distance(&current.valve, &target.valve).map(|distance| {
            instant + distance + 1
        }).and_then(|i| {
            if i > TIME { None } else { Some((i, target.rate * ((TIME - i) as i32))) }
        })
    }

    fn step(&self, nodes: Vec<Node>, current: &Node) -> i32 {
        let mut processed = nodes.into_iter()
            .map(|target| {
                if target.valve != current.valve && !target.visited {
                    if let Some((i, f)) = self.val(current.instant, &current, &target) {
                        let tf = current.max_flow + f;
                        if tf > target.max_flow {
                            println!("current {} target {} new flow {}", current, target, tf);
                            Node { max_flow: tf, instant: i, ..target }
                        } else {
                            target
                        }
                    } else {
                        target
                    }
                } else {
                    target
                }
            })
            .collect::<Vec<_>>();
        for index in 0..processed.len() {
            if processed[index].valve == current.valve {
                processed[index].visited = true
            }
        }
        if let Some(target) = processed.iter()
            .filter(|n| !n.visited)
            //.sorted_by_key(|n| n.max_flow)
            .last() {
            self.step(processed.clone(), target)
        } else {
            processed.sort_by(|a, b| a.instant.cmp(&b.instant));
            println!("{:?}", processed);
            processed.iter().map(|n| n.max_flow).max().unwrap_or_default()
        }
    }

}



pub fn solution(it: InputIterator, part_two: bool) -> Ztr {
    let world = World::create(it);
    let nodes = world.nodes();
    let start_node = Node { valve: Valve('A', 'A'), rate: 0, instant: 0, max_flow: 0, visited: false };
   (if part_two {
        todo!()
    } else {
        world.step(nodes, &start_node)
    }).to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_DATA: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn ref_sol() {
        let world = World::create(& mut (_TEST_DATA.lines().map(|s| s.into())));
        let flow =
            vec![(Valve('D', 'D'), 20), (Valve('B', 'B'), 13), (Valve('J', 'J'), 21), (Valve('H', 'H'), 22), (Valve('E', 'E'), 3), (Valve('C', 'C'), 2)]
                .into_iter()
                .map(|(valve, rate)| Node { valve, rate, instant: 0, visited: false, max_flow: 0 })
                .try_fold((0, 0, Node { valve: Valve('A', 'A'), rate: 0, instant: 0, visited: false, max_flow: 0}), |(instant, flow, from), to| {
                    world.val(instant, &from, &to).map(|(i, f)| (i, flow + f, to))
                })
                .map(|(_, f, _)| f);
        assert_eq!(Some(1651), flow);
    }


    #[test]
    fn test1() {
        assert_eq!("1651", solution(&mut (_TEST_DATA.lines().map(|s| s.into())), false));
    }

    #[test]
    fn test2() {
        assert!(true);
    }
}
