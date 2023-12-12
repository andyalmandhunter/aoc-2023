use std::{cell::Cell, fs};

#[derive(Debug)]
struct Card {
    copies: Cell<i32>,
    winning: Vec<i32>,
    have: Vec<i32>,
}

fn parse_line(line: &str) -> Card {
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

    Card {
        copies: Cell::new(1),
        winning,
        have,
    }
}

fn process_cards(cards: &Vec<Card>) -> i32 {
    for i in 0..cards.len() {
        let Card {
            copies,
            have,
            winning,
        } = &cards[i];
        let wins = have.iter().filter(|n| winning.contains(n)).count();

        for j in 0..wins {
            let Card { copies: n, .. } = &cards[i + j + 1];
            n.set(n.get() + copies.get());
        }
    }

    cards.iter().map(|Card { copies, .. }| copies.get()).sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let cards: Vec<Card> = input.lines().map(parse_line).collect();
    let answer = process_cards(&cards);

    println!("answer: {answer}");
}
