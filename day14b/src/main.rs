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

fn tilt_north(platform: &mut Vec<Vec<Space>>) {
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

fn tilt_west(platform: &mut Vec<Vec<Space>>) {
    for j in 0..platform.len() {
        let mut a = 0;
        let mut b = 0;

        loop {
            match platform[j][b] {
                Space::Empty => {
                    b += 1;
                }
                Space::Cube => {
                    b += 1;
                    a = b;
                }
                Space::Round => {
                    if b > a {
                        let mut temp = mem::replace(&mut platform[j][b], Space::Empty);
                        mem::swap(&mut temp, &mut platform[j][a]);
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

fn tilt_south(platform: &mut Vec<Vec<Space>>) {
    for j in 0..platform[0].len() {
        let mut a = platform.len() - 1;
        let mut b = platform.len() - 1;

        loop {
            match platform[b][j] {
                Space::Empty => {
                    if b == 0 {
                        break;
                    }
                    b -= 1;
                }
                Space::Cube => {
                    if b == 0 {
                        break;
                    }
                    b -= 1;
                    a = b;
                }
                Space::Round => {
                    if b < a {
                        let mut temp = mem::replace(&mut platform[b][j], Space::Empty);
                        mem::swap(&mut temp, &mut platform[a][j]);
                        if b == 0 {
                            break;
                        }
                        b -= 1;
                        a -= 1;
                    } else {
                        if b == 0 {
                            break;
                        }
                        b -= 1;
                        a = b;
                    }
                }
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<Space>>) {
    for j in 0..platform[0].len() {
        let mut a = platform.len() - 1;
        let mut b = platform.len() - 1;

        loop {
            match platform[j][b] {
                Space::Empty => {
                    if b == 0 {
                        break;
                    }
                    b -= 1;
                }
                Space::Cube => {
                    if b == 0 {
                        break;
                    }
                    b -= 1;
                    a = b;
                }
                Space::Round => {
                    if b < a {
                        let mut temp = mem::replace(&mut platform[j][b], Space::Empty);
                        mem::swap(&mut temp, &mut platform[j][a]);
                        if b == 0 {
                            break;
                        }
                        b -= 1;
                        a -= 1;
                    } else {
                        if b == 0 {
                            break;
                        }
                        b -= 1;
                        a = b;
                    }
                }
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

    for _ in 0..20_000 {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        println!("{}", total_load(&platform));
    }

    let answer = total_load(&platform);

    println!("answer: {answer}");
}
