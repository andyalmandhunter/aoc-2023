use std::fs;

fn parse_layout(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn new_visited(input: &str) -> Vec<Vec<(bool, bool, bool, bool)>> {
    input
        .lines()
        .map(|l| l.chars().map(|_| (false, false, false, false)).collect())
        .collect()
}

#[derive(Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn trace_beam(
    i: usize,
    j: usize,
    initial_direction: Dir,
    visited: &mut Vec<Vec<(bool, bool, bool, bool)>>,
    layout: &Vec<Vec<char>>,
    size: (usize, usize),
) {
    let mut k = i;
    let mut l = j;
    let mut d = initial_direction.clone();
    loop {
        match d {
            Dir::N => {
                if visited[k][l].0 {
                    break;
                }
                visited[k][l].0 = true;
            }
            Dir::E => {
                if visited[k][l].1 {
                    break;
                }
                visited[k][l].1 = true;
            }
            Dir::S => {
                if visited[k][l].2 {
                    break;
                }
                visited[k][l].2 = true;
            }
            Dir::W => {
                if visited[k][l].3 {
                    break;
                }
                visited[k][l].3 = true;
            }
        }

        match layout[k][l] {
            '.' => match d {
                Dir::N => {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                }
                Dir::E => {
                    if l == size.1 - 1 {
                        break;
                    }
                    l += 1;
                }
                Dir::S => {
                    if k == size.0 - 1 {
                        break;
                    }
                    k += 1;
                }
                Dir::W => {
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                }
            },
            '/' => match d {
                Dir::N => {
                    if l == size.1 - 1 {
                        break;
                    }
                    l += 1;
                    d = Dir::E;
                }
                Dir::E => {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                    d = Dir::N;
                }
                Dir::S => {
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    d = Dir::W;
                }
                Dir::W => {
                    if k == size.0 - 1 {
                        break;
                    }
                    k += 1;
                    d = Dir::S;
                }
            },
            '\\' => match d {
                Dir::N => {
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    d = Dir::W;
                }
                Dir::E => {
                    if k == size.0 - 1 {
                        break;
                    }
                    k += 1;
                    d = Dir::S;
                }
                Dir::S => {
                    if l == size.1 - 1 {
                        break;
                    }
                    l += 1;
                    d = Dir::E;
                }
                Dir::W => {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                    d = Dir::N;
                }
            },
            '-' => match d {
                Dir::N | Dir::S => {
                    trace_beam(k, l, Dir::E, visited, layout, size);
                    trace_beam(k, l, Dir::W, visited, layout, size);
                }
                Dir::E => {
                    if l == size.1 - 1 {
                        break;
                    }
                    l += 1;
                }
                Dir::W => {
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                }
            },
            '|' => match d {
                Dir::E | Dir::W => {
                    trace_beam(k, l, Dir::N, visited, layout, size);
                    trace_beam(k, l, Dir::S, visited, layout, size);
                }
                Dir::N => {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                }
                Dir::S => {
                    if k == size.0 - 1 {
                        break;
                    }
                    k += 1;
                }
            },
            _ => panic!(),
        }
    }
}

fn count_energized(visited: &Vec<Vec<(bool, bool, bool, bool)>>) -> usize {
    visited
        .iter()
        .flat_map(|row| row.iter().filter(|(n, s, e, w)| *n || *s || *e || *w))
        .count()
}

fn energized_str(visited: &Vec<Vec<(bool, bool, bool, bool)>>) -> Vec<String> {
    visited
        .iter()
        .map(|row| {
            row.iter()
                .map(|(n, e, s, w)| if *n || *e || *s || *w { '#' } else { '.' })
                .collect()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let layout = parse_layout(&input);
    let mut visited = new_visited(&input);
    trace_beam(
        0,
        0,
        Dir::E,
        &mut visited,
        &layout,
        (layout.len(), layout[0].len()),
    );

    for row in energized_str(&visited) {
        println!("{row}");
    }

    let answer = count_energized(&visited);

    eprintln!("answer: {answer}");
}
