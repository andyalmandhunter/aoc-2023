use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '.' | 'S' => Some(true),
                    '#' => Some(false),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn find_start(input: &str) -> Option<(usize, usize)> {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return Some((i, j));
            }
        }
    }

    None
}

fn count_reachable_points(map: &Vec<Vec<bool>>, start: (usize, usize), steps: usize) -> usize {
    let mut count_reachable = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut unvisited: BTreeSet<(usize, (usize, usize))> = BTreeSet::new();
    unvisited.insert((steps, start));

    while let Some((s, (i, j))) = unvisited.pop_last() {
        visited.insert((i, j));

        if s % 2 == 0 {
            count_reachable += 1;
        }

        if s == 0 {
            continue;
        }

        let up = i.checked_sub(1).map(|k| (k, j));
        let down = Some((i + 1, j));
        let right = Some((i, j + 1));
        let left = j.checked_sub(1).map(|l| (i, l));

        for (k, l) in [up, down, left, right].iter().flatten() {
            if let Some(row) = map.get(*k) {
                if let Some(tile) = row.get(*l) {
                    if *tile {
                        if !visited.contains(&(*k, *l)) {
                            unvisited.insert((s - 1, (*k, *l)));
                        }
                    }
                }
            }
        }
    }

    count_reachable
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let map = parse_input(&input);
    let start = find_start(&input).expect("unable to find start");

    let answer = count_reachable_points(&map, start, 64);
    println!("answer: {answer}");
}
