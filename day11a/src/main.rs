use std::fs;

fn get_locations(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(j, c)| {
                    if c == '#' {
                        Some((i as i64, j as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn adjust_locations(locations: &mut Vec<(i64, i64)>) {
    let max_row = locations.iter().map(|l| l.0).max().unwrap();
    let max_col = locations.iter().map(|l| l.1).max().unwrap();

    for i in (0..max_row).rev() {
        if locations.iter().find(|(x, _)| x == &i).is_none() {
            for (x, _) in locations.iter_mut() {
                if *x > i {
                    *x += 999_999;
                }
            }
        }
    }

    for i in (0..max_col).rev() {
        if locations.iter().find(|(_, x)| x == &i).is_none() {
            for (_, x) in locations.iter_mut() {
                if *x > i {
                    *x += 999_999;
                }
            }
        }
    }
}

fn shortest_distances(locations: &Vec<(i64, i64)>) -> i64 {
    let mut total_distance = 0;
    for i in 0..locations.len() {
        for j in 0..locations.len() {
            let (r0, c0) = locations[i];
            let (r1, c1) = locations[j];

            total_distance += (r1 - r0).abs() + (c1 - c0).abs();
        }
    }

    total_distance / 2
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let mut locations = get_locations(&input);
    adjust_locations(&mut locations);
    let answer = shortest_distances(&locations);

    println!("answer: {answer}");
}
