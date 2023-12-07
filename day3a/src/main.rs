use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

fn contains_symbol(substr: &&str, start: usize, end: usize) -> bool {
    substr[start..end]
        .chars()
        .find(|c| !c.is_digit(10) && c != &'.')
        .is_some()
}

fn find_in_line(line: &str, prev: Option<&&str>, next: Option<&&str>) -> Vec<i32> {
    print!("{line}");

    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    PATTERN
        .find_iter(line)
        .flat_map(|m| {
            let start = if m.start() < 1 { 0 } else { m.start() - 1 };
            let end = if m.end() >= line.len() {
                line.len()
            } else {
                m.end() + 1
            };

            if contains_symbol(&line, start, end) {
                return Some(m.as_str());
            }

            if let Some(p) = prev {
                if contains_symbol(p, start, end) {
                    return Some(m.as_str());
                }
            }

            if let Some(n) = next {
                if contains_symbol(n, start, end) {
                    return Some(m.as_str());
                }
            }

            None
        })
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn find_part_numbers(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    (0..lines.len())
        .flat_map(|i| {
            let line = lines[i];
            let prev = if i > 0 { lines.get(i - 1) } else { None };
            let next = lines.get(i + 1);

            let part_numbers = find_in_line(line, prev, next);
            println!("  part numbers: {:?}", part_numbers);

            part_numbers
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");

    let start = std::time::Instant::now();
    let answer: i32 = find_part_numbers(&input).iter().sum();
    let duration = start.elapsed();
    println!("got answer in {:?}", duration);

    println!("answer: {answer}");
}
