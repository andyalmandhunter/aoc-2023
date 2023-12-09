use once_cell::sync::Lazy;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::fs;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(d: char) -> Option<Direction> {
        match d {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

fn parse_node(line: &str) -> (&str, Node) {
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)$").unwrap());

    let caps = PATTERN.captures(line).expect("weird node line");

    let key = caps.get(1).unwrap().as_str();
    let left = caps.get(2).unwrap().as_str();
    let right = caps.get(3).unwrap().as_str();

    (key, Node { left, right })
}

fn parse_input<'a>(input: &'a String) -> (Vec<Direction>, FxHashMap<&'a str, Node<'a>>) {
    let mut lines = input.lines();

    let directions_str = lines.next().expect("no directions");
    let directions: Vec<_> = directions_str
        .chars()
        .map(|d| Direction::new(d).unwrap())
        .collect();

    lines.next();

    let node_map = FxHashMap::from_iter(lines.map(parse_node));

    (directions, node_map)
}

fn follow_map(directions: Vec<Direction>, nodes: FxHashMap<&str, Node>) -> i64 {
    let mut current_nodes: Vec<_> = nodes
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .collect();

    let mut steps: i64 = 0;
    for d in directions.iter().cycle() {
        if current_nodes
            .iter()
            .all(|n| n.chars().nth(2).unwrap() == 'Z')
        {
            break;
        }

        // if steps >= 1000 {
        //     break;
        // }

        if steps % 50_000_000 == 0 {
            println!("steps: {steps}");
        }

        for i in 0..current_nodes.len() {
            let n = nodes.get(current_nodes[i]).unwrap();
            current_nodes[i] = match d {
                Direction::Left => &n.left,
                Direction::Right => &n.right,
            }
        }

        steps += 1;
    }

    println!("nodes: {:?}", current_nodes);

    return steps;
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");

    let (directions, nodes) = parse_input(&input);
    let answer = follow_map(directions, nodes);

    // println!("directions: {:?}", directions);
    // println!("nodes: {:?}", nodes);

    println!("answer: {answer}");
}
