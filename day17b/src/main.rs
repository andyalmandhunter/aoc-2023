use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Visit {
    i: usize,
    j: usize,
    dir: Dir,
    run: usize,
}

fn get_neighbors(
    map: &Vec<Vec<u32>>,
    shape: (usize, usize),
    visited: &HashSet<Visit>,
    i: usize,
    j: usize,
    dir: &Dir,
    run: usize,
    heat_loss: u32,
) -> Vec<(u32, Visit)> {
    static MIN: usize = 4;
    static MAX: usize = 10;
    let mut unvisited = Vec::new();
    match dir {
        Dir::E => {
            if run < MAX && j < shape.1 - 1 {
                let v = Visit {
                    i,
                    j: j + 1,
                    dir: Dir::E,
                    run: run + 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j + 1 == shape.1 - 1) || run + 1 >= MIN {
                        unvisited.push((heat_loss + map[i][j + 1], v));
                    }
                }
            }
            if run >= MIN && i > 0 {
                let v = Visit {
                    i: i - 1,
                    j,
                    dir: Dir::N,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 && j == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i - 1][j], v));
                    }
                }
            }
            if run >= MIN && i < shape.0 - 1 {
                let v = Visit {
                    i: i + 1,
                    j,
                    dir: Dir::S,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i + 1 == shape.0 - 1 && j == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i + 1][j], v));
                    }
                }
            }
        }
        Dir::W => {
            if run < MAX && j > 0 {
                let v = Visit {
                    i,
                    j: j - 1,
                    dir: Dir::W,
                    run: run + 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j == shape.1) || run + 1 >= MIN {
                        unvisited.push((heat_loss + map[i][j - 1], v));
                    }
                }
            }
            if run >= MIN && i > 0 {
                let v = Visit {
                    i: i - 1,
                    j,
                    dir: Dir::N,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 && j == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i - 1][j], v));
                    }
                }
            }
            if run >= MIN && i < shape.0 - 1 {
                let v = Visit {
                    i: i + 1,
                    j,
                    dir: Dir::S,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i + 1 == shape.0 - 1 && j == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i + 1][j], v));
                    }
                }
            }
        }
        Dir::N => {
            if run < MAX && i > 0 {
                let v = Visit {
                    i: i - 1,
                    j,
                    dir: Dir::N,
                    run: run + 1,
                };
                if !visited.contains(&v) {
                    if !(i - 1 == shape.0 - 1 && j == shape.1 - 1) || run + 1 >= MIN {
                        unvisited.push((heat_loss + map[i - 1][j], v));
                    }
                }
            }
            if run >= MIN && j > 0 {
                let v = Visit {
                    i,
                    j: j - 1,
                    dir: Dir::W,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j == shape.1) {
                        unvisited.push((heat_loss + map[i][j - 1], v));
                    }
                }
            }
            if run >= MIN && j < shape.1 - 1 {
                let v = Visit {
                    i,
                    j: j + 1,
                    dir: Dir::E,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j + 1 == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i][j + 1], v));
                    }
                }
            }
        }
        Dir::S => {
            if run < MAX && i < shape.0 - 1 {
                let v = Visit {
                    i: i + 1,
                    j,
                    dir: Dir::S,
                    run: run + 1,
                };
                if !visited.contains(&v) {
                    if !(i + 1 == shape.0 - 1 && j == shape.1 - 1) || run + 1 >= MIN {
                        unvisited.push((heat_loss + map[i + 1][j], v));
                    }
                }
            }
            if run >= MIN && j > 0 {
                let v = Visit {
                    i,
                    j: j - 1,
                    dir: Dir::W,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j == shape.1) {
                        unvisited.push((heat_loss + map[i][j - 1], v));
                    }
                }
            }
            if run >= MIN && j < shape.1 - 1 {
                let v = Visit {
                    i,
                    j: j + 1,
                    dir: Dir::E,
                    run: 1,
                };
                if !visited.contains(&v) {
                    if !(i == shape.0 - 1 && j + 1 == shape.1 - 1) {
                        unvisited.push((heat_loss + map[i][j + 1], v));
                    }
                }
            }
        }
    }

    unvisited
}

fn min_heat_loss(map: &Vec<Vec<u32>>) -> u32 {
    let mut visited: HashSet<Visit> = HashSet::new();
    let mut unvisited: BTreeSet<(u32, Visit)> = BTreeSet::new();
    let mut heat_loss: HashMap<Visit, u32> = HashMap::new();

    let shape = (map.len(), map[0].len());

    let mut h = 0;
    let mut v = Visit {
        i: 0,
        j: 0,
        dir: Dir::E,
        run: 0,
    };

    loop {
        if v.i == shape.0 - 1 && v.j == shape.0 - 1 {
            break;
        }
        let neighbors = get_neighbors(&map, shape, &visited, v.i, v.j, &v.dir, v.run, h);

        // update unvisited and heat_loss
        for n in neighbors {
            if let Some(h_old) = heat_loss.get(&n.1) {
                if &n.0 < h_old {
                    heat_loss.insert(n.1.clone(), n.0);
                    unvisited.insert((n.0, n.1));
                }
            } else {
                heat_loss.insert(n.1.clone(), n.0);
                unvisited.insert((n.0, n.1));
            }
        }

        // done! mark the current node visited
        visited.insert(v.clone());

        // find the next node to visit
        let next = unvisited.pop_first().unwrap();
        h = next.0;
        v.i = next.1.i;
        v.j = next.1.j;
        v.dir = next.1.dir;
        v.run = next.1.run;
    }

    h
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let map = parse_input(&input);
    let answer = min_heat_loss(&map);

    println!("answer: {answer}")
}
