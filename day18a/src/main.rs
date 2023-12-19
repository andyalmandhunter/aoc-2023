use once_cell::sync::Lazy;
use regex::{Match, Regex};
use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
enum Dir {
    U,
    D,
    R,
    L,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    length: u32,
    _color: (u8, u8, u8),
}

fn byte_from_match(m: Match) -> Option<u8> {
    u8::from_str_radix(m.as_str(), 16).ok()
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .lines()
        .flat_map(|l| -> Option<Step> {
            static PATTERN: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"([UDRL]) (\d+) \(#(.{2})(.{2})(.{2})\)").unwrap());

            let captures = PATTERN.captures(l)?;
            let dir = captures.get(1).and_then(|c| match c.as_str() {
                "U" => Some(Dir::U),
                "D" => Some(Dir::D),
                "R" => Some(Dir::R),
                "L" => Some(Dir::L),
                _ => None,
            })?;
            let length = captures
                .get(2)
                .and_then(|c| c.as_str().parse::<u32>().ok())?;
            let r = captures.get(3).and_then(byte_from_match)?;
            let g = captures.get(4).and_then(byte_from_match)?;
            let b = captures.get(5).and_then(byte_from_match)?;

            Some(Step {
                dir,
                length,
                _color: (r, g, b),
            })
        })
        .collect()
}

fn dig_trench(steps: &[Step]) -> Vec<(i32, i32, Dir)> {
    let mut trench: Vec<(i32, i32, Dir)> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    for s in steps {
        trench.push((i, j, s.dir.clone()));
        let (di, dj) = match s.dir {
            Dir::U => (-1, 0),
            Dir::D => (1, 0),
            Dir::R => (0, 1),
            Dir::L => (0, -1),
        };

        for _ in 0..s.length {
            i += di;
            j += dj;
            trench.push((i, j, s.dir.clone()));
        }
    }
    trench
}

#[derive(Debug)]
enum E {
    U,
    D,
    R,
    L,
    UR,
    RD,
    DL,
    LU,
    DR,
    RU,
    UL,
    LD,
    None,
}

fn get_path(trench: &Vec<(i32, i32, Dir)>) -> Option<Vec<String>> {
    let i_min = *trench.iter().map(|(i, _, _)| i).min()?;
    let i_max = *trench.iter().map(|(i, _, _)| i).max()?;
    let j_min = *trench.iter().map(|(_, j, _)| j).min()?;
    let j_max = *trench.iter().map(|(_, j, _)| j).max()?;

    let mut map: Vec<Vec<E>> = (i_min..=i_max)
        .map(|_| (j_min..=j_max).map(|_| E::None).collect())
        .collect();

    for (i, j, d) in trench {
        let k = (i - i_min) as usize;
        let l = (j - j_min) as usize;
        match map[k][l] {
            E::None => match d {
                Dir::U => map[k][l] = E::U,
                Dir::D => map[k][l] = E::D,
                Dir::R => map[k][l] = E::R,
                Dir::L => map[k][l] = E::L,
            },
            E::U => match d {
                Dir::R => map[k][l] = E::UR,
                Dir::L => map[k][l] = E::UL,
                _ => return None,
            },
            E::D => match d {
                Dir::R => map[k][l] = E::DR,
                Dir::L => map[k][l] = E::DL,
                _ => return None,
            },
            E::R => match d {
                Dir::U => map[k][l] = E::RU,
                Dir::D => map[k][l] = E::RD,
                _ => return None,
            },
            E::L => match d {
                Dir::U => map[k][l] = E::LU,
                Dir::D => map[k][l] = E::LD,
                _ => return None,
            },
            _ => return None,
        }
    }

    let mut map_chars: Vec<Vec<char>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|e| match e {
                    E::U | E::D => '|',
                    E::R | E::L => '-',
                    E::UR | E::LD => 'F',
                    E::RD | E::UL => '7',
                    E::DL | E::RU => 'J',
                    E::LU | E::DR => 'L',
                    E::None => '.',
                })
                .collect()
        })
        .collect();

    // fix start
    let i_start = (trench[0].0 - i_min) as usize;
    let j_start = (trench[0].1 - j_min) as usize;
    map_chars[i_start][j_start] = '7';

    Some(map_chars.iter().map(|row| row.iter().collect()).collect())
}

fn interior_area(path: &[String]) -> usize {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\||F-*J|L-*7)").unwrap());

    path.iter()
        .flat_map(|line| {
            line.chars().enumerate().filter(|(i, c)| {
                if c == &'.' {
                    let substr = &line[0..*i];
                    let crossings = PATTERN.find_iter(substr).count();
                    crossings % 2 == 1
                } else {
                    false
                }
            })
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let steps = parse_input(&input);
    let trench = dig_trench(&steps);
    let perimeter = trench
        .iter()
        .map(|x| (x.0, x.1))
        .collect::<HashSet<_>>()
        .len();
    let path = get_path(&trench).expect("unable to get path");

    for line in &path {
        println!("{line}");
    }

    let interior = interior_area(&path);
    let answer = perimeter + interior;

    println!("answer: {answer}");
}
