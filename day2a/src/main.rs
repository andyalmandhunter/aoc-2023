use std::fs;

fn parse_id(raw_id_str: &str) -> i32 {
    let mut parts = raw_id_str.split_whitespace();
    parts.next();

    let id_str = parts.next().unwrap();
    id_str.parse::<i32>().expect("unable to parse game ID")
}

fn is_draw_possible(draw_str: &str) -> bool {
    let mut draw_parts = draw_str.split_whitespace();
    let n = draw_parts.next().unwrap().parse::<i32>().unwrap();
    let color = draw_parts.next().unwrap();

    match color {
        "red" => n <= 12,
        "green" => n <= 13,
        "blue" => n <= 14,
        _ => false,
    }
}

fn is_hand_possible(hand_str: &str) -> bool {
    let mut draws = hand_str.split(',');
    draws.all(is_draw_possible)
}

fn is_possible(game_str: &str) -> Option<i32> {
    let mut game_and_hands = game_str.split(':');
    let game_id = parse_id(game_and_hands.next().unwrap());

    let hands_str = game_and_hands.next().unwrap();
    let mut hands = hands_str.split(';');

    if hands.all(is_hand_possible) {
        Some(game_id)
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer: i32 = input.lines().flat_map(is_possible).sum();

    println!("answer: {answer}");
}
