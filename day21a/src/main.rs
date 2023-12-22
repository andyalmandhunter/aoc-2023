use std::{collections::HashSet, fs};

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
    let mut visited: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut unvisited: Vec<((usize, usize), usize)> = Vec::new();
    unvisited.push((start, steps));

    while let Some(((i, j), s)) = unvisited.pop() {
        if visited.contains(&((i, j), s)) {
            continue;
        }

        visited.insert(((i, j), s));

        if s == 0 {
            count_reachable += 1;
            continue;
        }

        let up = i.checked_sub(1).and_then(|k| {
            map.get(k).and_then(|row| {
                row.get(j)
                    .and_then(|s| if *s { Some((k, j)) } else { None })
            })
        });
        let down = map.get(i + 1).and_then(|row| {
            row.get(j)
                .and_then(|s| if *s { Some((i + 1, j)) } else { None })
        });
        let right = map.get(i).and_then(|row| {
            row.get(j + 1)
                .and_then(|s| if *s { Some((i, j + 1)) } else { None })
        });
        let left = map.get(i).and_then(|row| {
            j.checked_sub(1).and_then(|l| {
                row.get(l)
                    .and_then(|s| if *s { Some((i, l)) } else { None })
            })
        });

        for (k, l) in [up, down, left, right].iter().flatten() {
            let next = ((*k, *l), s - 1);
            unvisited.push(next);
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
