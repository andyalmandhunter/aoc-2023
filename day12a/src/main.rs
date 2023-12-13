use bitvec::prelude::*;
// use once_cell::sync::Lazy;
// use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Record {
    conds: Vec<Cond>,
    groups: Vec<u32>,
}

#[derive(Debug, PartialEq)]
enum Cond {
    O, // operational
    B, // broken
    U, // unknown
}

fn cond_product(n: u32) -> impl Iterator<Item = Vec<Cond>> {
    (0_u32..(2_u32.pow(n))).map(move |i| {
        let bits = BitArray::<_, Lsb0>::new(i);
        bits.iter()
            .take(n as usize)
            .map(|b| if b == true { Cond::O } else { Cond::B })
            .collect::<Vec<_>>()
    })
}

#[test]
fn test_cond_product() {
    for c in cond_product(5) {
        println!("{:?}", c);
    }

    let mut p = cond_product(3);
    assert_eq!(p.next(), Some(vec![Cond::B, Cond::B, Cond::B]));
    assert_eq!(p.next(), Some(vec![Cond::O, Cond::B, Cond::B]));
    assert_eq!(p.next(), Some(vec![Cond::B, Cond::O, Cond::B]));
    assert_eq!(p.next(), Some(vec![Cond::O, Cond::O, Cond::B]));
    assert_eq!(p.next(), Some(vec![Cond::B, Cond::B, Cond::O]));
    assert_eq!(p.next(), Some(vec![Cond::O, Cond::B, Cond::O]));
    assert_eq!(p.next(), Some(vec![Cond::B, Cond::O, Cond::O]));
    assert_eq!(p.next(), Some(vec![Cond::O, Cond::O, Cond::O]));
    assert_eq!(p.next(), None);
}

fn parse_record(line: &str) -> Record {
    let mut parts = line.split_whitespace();

    let mut raw_conds_str = parts.next().expect("no conditions").to_string();
    raw_conds_str.push('?');
    let mut conds_str = raw_conds_str.repeat(5);
    conds_str.pop();
    let conds: Vec<_> = conds_str
        .chars()
        .flat_map(|c| match c {
            '#' => Some(Cond::B),
            '.' => Some(Cond::O),
            '?' => Some(Cond::U),
            _ => None,
        })
        .collect();

    let groups_str = parts.next().expect("no groups");
    let groups = groups_str
        .split(',')
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>()
        .repeat(5);

    Record { conds, groups }
}

fn possible_arrangments(record: &Record) -> u32 {
    let unknown_count = record.conds.iter().filter(|c| c == &&Cond::U).count();
    let possible_arrangements = cond_product(unknown_count as u32);
    possible_arrangements
        .filter(|a| check_arrangement(&record.conds, a, &record.groups))
        .count()
        .try_into()
        .unwrap()
}

fn check_arrangement(conds: &[Cond], unknowns: &[Cond], groups: &[u32]) -> bool {
    let mut i = 0;
    let arrangement: String = conds
        .iter()
        .map(|c| match c {
            Cond::U => {
                i += 1;
                match &unknowns[i - 1] {
                    Cond::O => ' ',
                    Cond::B => 'B',
                    _ => panic!(),
                }
            }
            Cond::O => ' ',
            Cond::B => 'B',
        })
        .collect();

    // static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"B+").unwrap());

    let arrangement_groups: Vec<_> = arrangement
        .split_whitespace()
        .map(|s| s.len() as u32)
        .collect();
    // .find_iter(&arrangement)
    // .map(|m| m.len() as u32)
    // .collect();

    // println!("{arrangement}");
    // println!("arrang: {:?}", arrangement_groups);
    // println!("groups: {:?}", groups);
    // println!("");

    let m = &arrangement_groups == groups;

    // if m {
    //     println!("{arrangement}");
    // }

    m
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let records: Vec<_> = input.lines().map(parse_record).collect();

    for r in &records {
        println!("{:?}", r);
    }

    let answer: u32 = records.iter().map(possible_arrangments).sum();

    println!("answer: {answer}");
}
