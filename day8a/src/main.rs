use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, fs};

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
        Lazy::new(|| Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap());

    let caps = PATTERN.captures(line).expect("weird node line");

    let key = caps.get(1).unwrap().as_str();
    let left = caps.get(2).unwrap().as_str();
    let right = caps.get(3).unwrap().as_str();

    (key, Node { left, right })
}

fn parse_input<'a>(input: &'a String) -> (Vec<Direction>, HashMap<&'a str, Node<'a>>) {
    let mut lines = input.lines();

    let directions_str = lines.next().expect("no directions");
    let directions: Vec<_> = directions_str
        .chars()
        .map(|d| Direction::new(d).unwrap())
        .collect();

    lines.next();

    let node_map = HashMap::from_iter(lines.map(parse_node));

    (directions, node_map)
}

fn follow_map(directions: Vec<Direction>, nodes: HashMap<&str, Node>) -> i32 {
    let mut current_node = "AAA";
    let mut steps = 0;
    for d in directions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        let n = nodes.get(current_node).unwrap();
        current_node = match d {
            Direction::Left => n.left,
            Direction::Right => n.right,
        };
        steps += 1;
    }
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
