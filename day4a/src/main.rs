use std::fs;

fn parse_line(line: &str) -> (Vec<i32>, Vec<i32>) {
    let mut parts = line.split(':');
    parts.next();

    let mut numbers = parts.next().unwrap().split('|');

    let winning_str = numbers.next().expect("no winning numbers");
    let have_str = numbers.next().expect("no have numbers");

    let winning: Vec<_> = winning_str
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let have: Vec<_> = have_str
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // println!("winning: {:?}, have: {:?}", winning, have);

    (winning, have)
}

fn score(winning: &[i32], have: &[i32]) -> i32 {
    let wins = have.iter().filter(|n| winning.contains(n)).count();

    if wins == 0 {
        0
    } else {
        2_i32.pow(wins as u32 - 1)
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer: i32 = input
        .lines()
        .map(parse_line)
        .map(|(w, h)| score(&w, &h))
        .sum();

    println!("answer: {answer}");
}
