use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    Start,
}

impl Tile {
    fn new(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::NS),
            '-' => Some(Self::EW),
            'L' => Some(Self::NE),
            'J' => Some(Self::NW),
            '7' => Some(Self::SW),
            'F' => Some(Self::SE),
            '.' => Some(Self::G),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| Tile::new(c).unwrap()).collect()
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(parse_line).collect::<Vec<_>>()
}

fn loop_path(sketch: Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let (mut i, mut j) = find_start(&sketch).expect("unable to find start");

    println!("start: {:?}", (i, j));

    let mut next_tiles = vec![
        (i, j + 1, Tile::EW),
        (i, j + 1, Tile::NW),
        (i, j + 1, Tile::SW),
        (i + 1, j, Tile::NS),
        (i + 1, j, Tile::NE),
        (i + 1, j, Tile::NW),
    ];

    if i > 0 {
        next_tiles.append(&mut vec![
            (i - 1, j, Tile::NS),
            (i - 1, j, Tile::SW),
            (i - 1, j, Tile::SE),
        ])
    }

    if j > 0 {
        next_tiles.append(&mut vec![
            (i, j - 1, Tile::EW),
            (i, j - 1, Tile::NE),
            (i, j - 1, Tile::SE),
        ])
    }

    let (mut k, mut l, _) = next_tiles
        .iter()
        .find(|(k, l, t)| {
            sketch
                .get(*k)
                .and_then(|o| o.get(*l))
                .is_some_and(|tt| tt == t)
        })
        .expect("unable to find direction from start");

    let mut path = Vec::new();
    loop {
        path.push((i, j));

        // println!("next: {:?}", (k, l));

        let tile = sketch
            .get(k)
            .and_then(|o| o.get(l))
            .expect("next tile does not exist");

        if tile == &Tile::Start {
            break;
        }

        let (m, n) = match (k as i32 - i as i32, l as i32 - j as i32) {
            (-1, 0) => match tile {
                Tile::NS => (k - 1, l),
                Tile::SW => (k, l - 1),
                Tile::SE => (k, l + 1),
                _ => panic!(),
            },
            (0, 1) => match tile {
                Tile::EW => (k, l + 1),
                Tile::NW => (k - 1, l),
                Tile::SW => (k + 1, l),
                _ => panic!(),
            },
            (1, 0) => match tile {
                Tile::NS => (k + 1, l),
                Tile::NE => (k, l + 1),
                Tile::NW => (k, l - 1),
                _ => panic!(),
            },
            (0, -1) => match tile {
                Tile::EW => (k, l - 1),
                Tile::NE => (k - 1, l),
                Tile::SE => (k + 1, l),
                _ => panic!(),
            },
            _ => panic!(),
        };

        (i, j) = (k, l);
        (k, l) = (m, n);
    }

    path
}

fn find_start(sketch: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (i, row) in sketch.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile == &Tile::Start {
                return Some((i, j));
            }
        }
    }
    None
}

fn build_map(input: &str, path: Vec<(usize, usize)>) -> Vec<String> {
    let path_set = path.iter().collect::<HashSet<_>>();

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if path_set.contains(&(i, j)) {
                        if c == 'S' {
                            '-'
                            // '7'
                        } else {
                            c
                        }
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

fn count_enclosed(path_map: &Vec<String>) -> usize {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\||F-*J|L-*7)").unwrap());

    path_map
        .iter()
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

#[test]
fn test_things() {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\||F-*J|L-*7)").unwrap());
    let crossings = PATTERN.find_iter("...F-JF---7..|L7").count();
    assert_eq!(crossings, 3);
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let sketch = parse_input(&input);
    let path = loop_path(sketch);
    let path_map = build_map(&input, path);

    println!("{}", path_map.join("\n"));

    let answer = count_enclosed(&path_map);

    println!("answer: {answer}");
}
