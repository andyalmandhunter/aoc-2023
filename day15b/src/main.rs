use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, fs};

fn hash(val: &str) -> u32 {
    let mut current = 0;
    for n in val.chars().map(|c| c as u32) {
        current += n;
        current *= 17;
        current %= 256;
    }
    current
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
    assert_eq!(hash("rn"), 0);
    assert_eq!(hash("qp"), 1);
    assert_eq!(hash("cm"), 0);
}

#[derive(Debug, PartialEq)]
enum Step {
    Add(String, u32),
    Remove(String),
}

fn parse_step(step: &str) -> Option<Step> {
    static ADD: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)=(\d+)").unwrap());
    static REM: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)-").unwrap());

    if let Some(c) = ADD.captures(step) {
        let key = c.get(1).unwrap().as_str();
        let value = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
        Some(Step::Add(key.to_string(), value))
    } else if let Some(c) = REM.captures(step) {
        let key = c.get(1).unwrap().as_str();
        Some(Step::Remove(key.to_string()))
    } else {
        None
    }
}

#[test]
fn test_parse_step() {
    assert_eq!(parse_step(&"rn=1"), Some(Step::Add("rn".to_string(), 1)));
    assert_eq!(parse_step(&"cm-"), Some(Step::Remove("cm".to_string())));
}

fn install_lenses(steps: &[Step]) -> HashMap<u32, Vec<(String, u32)>> {
    let mut m: HashMap<u32, Vec<(String, u32)>> = HashMap::new();
    for step in steps {
        match step {
            Step::Add(k, v) => {
                let h = hash(k);
                if let Some(s) = m.get_mut(&h) {
                    if let Some(i) = s.iter().position(|(l, _)| l == k) {
                        s[i] = (k.to_string(), *v);
                    } else {
                        s.push((k.to_string(), *v));
                    }
                } else {
                    m.insert(h, vec![(k.to_string(), *v)]);
                }
            }
            Step::Remove(k) => {
                let h = hash(k);
                if let Some(s) = m.get_mut(&h) {
                    if let Some(i) = s.iter().position(|(l, _)| l == k) {
                        s.remove(i);
                    }
                }
                if m.get(&h).map(|s| s.len() == 0) == Some(true) {
                    m.remove(&h);
                }
            }
        }
    }

    m
}

fn focusing_power(boxes: &HashMap<u32, Vec<(String, u32)>>) -> u32 {
    boxes
        .iter()
        .flat_map(|(n, ls)| {
            ls.iter()
                .enumerate()
                .map(move |(i, (_, j))| (1 + n) * &(1 + i as u32) * j)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let steps: Vec<_> = input
        .split(',')
        .map(|s| s.trim())
        .flat_map(parse_step)
        .collect();
    let boxes = install_lenses(&steps);
    let answer: u32 = focusing_power(&boxes);

    println!("answer: {answer}");
}
