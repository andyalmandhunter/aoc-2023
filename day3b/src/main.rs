use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Bound::{Excluded, Included};
use std::{collections::BTreeMap, fs};

fn get_parts(input: &str, part_symbol: char) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(
                move |(j, c)| {
                    if c == part_symbol {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn get_part_numbers(input: &str) -> BTreeMap<(usize, (usize, usize)), u32> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    let mut part_nums = BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        for m in PATTERN.find_iter(line) {
            let num = m.as_str().parse::<u32>().unwrap();
            part_nums.insert((i, (m.start(), m.end())), num);
        }
    }

    part_nums
}

fn maybe_gear_ratio(
    part_nums: &BTreeMap<(usize, (usize, usize)), u32>,
    gear: &(usize, usize),
) -> Option<u32> {
    let (i, j) = gear;
    let min_i = if i > &0 { i - 1 } else { 0 };
    let adjacent_nums: Vec<_> = part_nums
        .range((Included((min_i, (0, 0))), Excluded((i + 2, (0, 0)))))
        .filter_map(|((_, (n, m)), part_num)| {
            if (&(n + 1) >= j && n <= &(j + 1)) || (m >= j && m <= &(j + 2)) {
                Some(part_num)
            } else {
                None
            }
        })
        .collect();

    if adjacent_nums.len() == 2 {
        println!("found gear: {:?}", (i, j));
        Some(adjacent_nums.iter().map(|n| **n).product())
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let maybe_gears = get_parts(&input, '*');
    let part_numbers = get_part_numbers(&input);

    let answer: u32 = maybe_gears
        .iter()
        .filter_map(|g| maybe_gear_ratio(&part_numbers, g))
        .sum();

    println!("answer: {answer}");
}
