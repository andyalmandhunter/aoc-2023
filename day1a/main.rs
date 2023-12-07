use std::fs;

fn digit(c: &char) -> bool {
    c.is_digit(10)
}

fn digits(line: &str) -> i32 {
    println!("{line}");

    let left_digit = line.chars().find(digit).expect("no left digit found");
    println!("left digit: {left_digit}");

    let right_digit = line.chars().rfind(digit).expect("no right digit found");
    println!("right digit: {right_digit}");

    let number = format!("{left_digit}{right_digit}")
        .parse::<i32>()
        .expect("unable to parse as integer");
    println!("{number}");

    number
}

fn main() {
    let input_path = "input";
    let input = fs::read_to_string(input_path).expect("unable to read input");

    let answer: i32 = input.lines().map(digits).sum();
    println!("answer: {answer}")
}
