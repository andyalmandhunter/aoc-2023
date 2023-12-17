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
    shape: (usize, usize),
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
                    if l == shape.1 - 1 {
                        break;
                    }
                    l += 1;
                }
                Dir::S => {
                    if k == shape.0 - 1 {
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
                    if l == shape.1 - 1 {
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
                    if k == shape.0 - 1 {
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
                    if k == shape.0 - 1 {
                        break;
                    }
                    k += 1;
                    d = Dir::S;
                }
                Dir::S => {
                    if l == shape.1 - 1 {
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
                    trace_beam(k, l, Dir::E, visited, layout, shape);
                    trace_beam(k, l, Dir::W, visited, layout, shape);
                }
                Dir::E => {
                    if l == shape.1 - 1 {
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
                    trace_beam(k, l, Dir::N, visited, layout, shape);
                    trace_beam(k, l, Dir::S, visited, layout, shape);
                }
                Dir::N => {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                }
                Dir::S => {
                    if k == shape.0 - 1 {
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

fn initial_beams(shape: (usize, usize)) -> impl Iterator<Item = (usize, usize, Dir)> {
    let top = (0..shape.1).map(|i| (0, i, Dir::S));
    let right = (0..shape.0).map(move |i| (i, shape.1 - 1, Dir::W));
    let bottom = (0..shape.1).map(move |i| (shape.0 - 1, i, Dir::N));
    let left = (0..shape.0).map(|i| (i, 0, Dir::E));
    top.chain(right).chain(bottom).chain(left)
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let layout = parse_layout(&input);

    let shape = (layout.len(), layout[0].len());
    let answer = initial_beams(shape)
        .map(|(i, j, d)| {
            let mut visited = new_visited(&input);
            trace_beam(i, j, d, &mut visited, &layout, shape);
            count_energized(&visited)
        })
        .max()
        .unwrap();

    eprintln!("answer: {answer}");
}
