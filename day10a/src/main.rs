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

fn loop_length(sketch: Vec<Vec<Tile>>) -> i64 {
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
        .find(|(k, l, _)| sketch.get(*k).and_then(|o| o.get(*l)).is_some())
        .expect("unable to find direction from start");

    let mut steps = 1;
    loop {
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
        steps += 1;
    }

    steps / 2
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

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let sketch = parse_input(&input);
    let answer = loop_length(sketch);

    println!("answer: {answer}");
}
