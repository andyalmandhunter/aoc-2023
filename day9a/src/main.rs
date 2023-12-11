use std::{fs, mem};

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

#[derive(Debug)]
struct Sequences {
    next: Vec<i32>,
}

impl Sequences {
    fn new(initial: Vec<i32>) -> Self {
        Sequences { next: initial }
    }
}

impl Iterator for Sequences {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        let mut next: Vec<_> = self.next.windows(2).map(|w| &w[1] - &w[0]).collect();
        mem::swap(&mut next, &mut self.next);
        Some(next)
    }
}

fn predict_next(sequence: Vec<i32>) -> i32 {
    let sequences: Vec<_> = Sequences::new(sequence)
        .take_while(|s| !s.iter().all(|x| x == &0))
        .collect();
    sequences.iter().flat_map(|s| s.last()).sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer: i32 = input.lines().map(parse_line).map(predict_next).sum();

    println!("answer: {answer}");
}
