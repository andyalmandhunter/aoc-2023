use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

fn parse<'a>(token: &'a str) -> &'a str {
    match token {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        t => t,
    }
}

fn find<I: Iterator<Item = usize>>(line: &str, range: I) -> Option<&str> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|0)").unwrap()
    });

    for i in range {
        let sub_line = &line[i..];
        if let Some(m) = PATTERN.find(sub_line) {
            return Some(m.as_str());
        }
    }

    None
}

fn find_forward(line: &str) -> Option<&str> {
    find(line, 0..line.len())
}

fn find_backward(line: &str) -> Option<&str> {
    find(line, (0..line.len()).rev())
}

fn digits(line: &str) -> i32 {
    // println!("{line}");

    let left_raw = find_forward(line).expect("no left digit found");
    let left_digit = parse(left_raw);
    // println!("left digit: {left_digit}");

    let right_raw = find_backward(line).expect("no right digit found");
    let right_digit = parse(right_raw);
    // println!("right digit: {right_digit}");

    let number = format!("{left_digit}{right_digit}")
        .parse::<i32>()
        .expect("unable to parse as integer");
    // println!("{number}");

    number
}

fn main() {
    let input_path = "input";
    let input = fs::read_to_string(input_path).expect("unable to read input");

    // let _: Vec<_> = input.lines().map(digits).collect();

    let start = std::time::Instant::now();
    let answer: i32 = input.lines().map(digits).sum();
    let duration = start.elapsed();
    println!("got answer in {:?}", duration);

    println!("answer: {answer}")
}
