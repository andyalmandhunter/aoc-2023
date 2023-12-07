use std::fs;

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Draw {
    color: Color,
    n: i32,
}

fn parse_draw(draw_str: &str) -> Option<Draw> {
    let mut draw_parts = draw_str.split_whitespace();
    let n = draw_parts.next().unwrap().parse::<i32>().unwrap();
    let color = draw_parts.next().unwrap();

    match color {
        "red" => Some(Draw {
            color: Color::Red,
            n,
        }),
        "green" => Some(Draw {
            color: Color::Green,
            n,
        }),
        "blue" => Some(Draw {
            color: Color::Blue,
            n,
        }),
        _ => None,
    }
}

fn parse_draws(hand_str: &str) -> Vec<Draw> {
    let draws = hand_str.split(',');
    draws.flat_map(parse_draw).collect()
}

fn max_draw(draws: &Vec<Draw>, color: Color) -> i32 {
    draws
        .iter()
        .filter_map(|d| if d.color == color { Some(d.n) } else { None })
        .max()
        .unwrap_or(0)
}

fn game_power(game_str: &str) -> i32 {
    let mut game_and_hands = game_str.split(':');
    game_and_hands.next();

    let hands_str = game_and_hands.next().unwrap();
    let hands = hands_str.split(';');

    let draws = hands.flat_map(parse_draws).collect();

    let max_red = max_draw(&draws, Color::Red);
    let max_green = max_draw(&draws, Color::Green);
    let max_blue = max_draw(&draws, Color::Blue);

    println!("{game_str}");
    println!("red: {:?}", max_red);
    println!("green: {:?}", max_green);
    println!("blue: {:?}", max_blue);

    max_red * max_green * max_blue
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer: i32 = input.lines().map(game_power).sum();

    println!("answer: {answer}");
}
