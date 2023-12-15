use std::fmt;
use std::fs;
use std::mem;

enum Space {
    Round,
    Cube,
    Empty,
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Space::Round => write!(f, "O"),
            Space::Cube => write!(f, "#"),
            Space::Empty => write!(f, "."),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    'O' => Some(Space::Round),
                    '#' => Some(Space::Cube),
                    '.' => Some(Space::Empty),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn tilt_up(platform: &mut Vec<Vec<Space>>) {
    for j in 0..platform[0].len() {
        let mut a = 0;
        let mut b = 0;

        loop {
            match platform[b][j] {
                Space::Empty => {
                    b += 1;
                }
                Space::Cube => {
                    b += 1;
                    a = b;
                }
                Space::Round => {
                    if b > a {
                        let mut temp = mem::replace(&mut platform[b][j], Space::Empty);
                        mem::swap(&mut temp, &mut platform[a][j]);
                        a += 1;
                        b += 1;
                    } else {
                        b += 1;
                        a = b;
                    }
                }
            }
            if b >= platform.len() {
                break;
            }
        }
    }
}

fn total_load(platform: &Vec<Vec<Space>>) -> usize {
    let rows = platform.len();
    platform
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().map(move |s| match s {
                Space::Round => rows - i,
                _ => 0,
            })
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let mut platform = parse(&input);

    for row in &platform {
        println!("{:?}", row);
    }

    tilt_up(&mut platform);

    println!("");
    for row in &platform {
        println!("{:?}", row);
    }

    let answer = total_load(&platform);

    println!("answer: {answer}");
}
