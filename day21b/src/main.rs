use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
    sync::{Mutex, OnceLock},
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

fn count_reachable(
    map: &Vec<Vec<bool>>,
    start: (usize, usize),
    even: bool,
    limit: Option<usize>,
) -> usize {
    let mut reachable = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut unvisited: BTreeSet<(usize, (usize, usize))> = BTreeSet::new();
    unvisited.insert((0, start));

    while let Some((s, (i, j))) = unvisited.pop_first() {
        if visited.contains(&(i, j)) {
            continue;
        }

        visited.insert((i, j));

        if even {
            if s % 2 == 0 {
                reachable += 1;
            }
        } else {
            if s % 2 == 1 {
                reachable += 1;
            }
        }

        if let Some(l) = limit {
            if s == l {
                continue;
            }
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
                            unvisited.insert((s + 1, (*k, *l)));
                        }
                    }
                }
            }
        }
    }

    reachable
}

fn count_reachable_limit(map: &Vec<Vec<bool>>, start: (usize, usize), limit: usize) -> usize {
    static CACHE: OnceLock<Mutex<HashMap<((usize, usize), usize), usize>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    if let Some(value) = cache.get(&(start, limit)) {
        // println!("returning cached value");
        *value
    } else {
        let value = count_reachable(map, start, limit % 2 == 0, Some(limit));
        cache.insert((start, limit), value);
        value
    }
}

fn count_reachable_nolimit(map: &Vec<Vec<bool>>, start: (usize, usize), even: bool) -> usize {
    static CACHE: OnceLock<Mutex<HashMap<((usize, usize), bool), usize>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    if let Some(value) = cache.get(&(start, even)) {
        // println!("returning cached value");
        *value
    } else {
        let value = count_reachable(map, start, even, None);
        cache.insert((start, even), value);
        value
    }
}

fn go_sideways(map: &Vec<Vec<bool>>, start: (usize, usize), limit: usize) -> usize {
    // figure out how many repetitions are fully reachable and can be computed
    // quickly

    let full: usize = match start {
        (65, _) => 195,
        _ => 260,
    };

    let n = if limit > full {
        (limit - full) / 131
    } else {
        0
    };

    let mut reachable = 0;
    reachable += (n / 2) * count_reachable_nolimit(map, start, true);
    reachable += (n / 2) * count_reachable_nolimit(map, start, false);
    if n % 2 == 1 {
        reachable += if limit % 2 == 0 {
            count_reachable_nolimit(map, start, true)
        } else {
            count_reachable_nolimit(map, start, false)
        };
    }

    let mut l = limit - (n * 131);
    loop {
        reachable += count_reachable_limit(map, start, l);

        if l >= 131 {
            l -= 131;
        } else {
            break;
        }
    }

    reachable
}

fn count_all_reachable(map: &Vec<Vec<bool>>) -> usize {
    static LIMIT: usize = 26_501_365;

    let mut reachable = 0;

    // start row: start plus go left plus go right
    reachable += count_reachable_limit(map, (65, 65), LIMIT);
    reachable += go_sideways(map, (65, 0), LIMIT - 66);
    reachable += go_sideways(map, (65, 130), LIMIT - 66);

    // rows above: sub 66, then repeatedly sub 131 until < 131 remain
    let mut l = LIMIT - 66;
    loop {
        if l >= 260 {
            reachable += count_reachable_nolimit(map, (130, 65), l % 2 == 0);
        } else {
            reachable += count_reachable_limit(map, (130, 65), l);
        }

        if l >= 66 {
            reachable += go_sideways(map, (130, 0), l - 66);
            reachable += go_sideways(map, (130, 130), l - 66);
        }

        if l >= 131 {
            l -= 131
        } else {
            break;
        }
    }

    // rows below: sub 66, then repeatedly sub 131 until < 131 remain
    let mut l = LIMIT - 66;
    loop {
        if l >= 260 {
            reachable += count_reachable_nolimit(map, (0, 65), l % 2 == 0);
        } else {
            reachable += count_reachable_limit(map, (0, 65), l);
        }

        if l >= 66 {
            reachable += go_sideways(map, (0, 0), l - 66);
            reachable += go_sideways(map, (0, 130), l - 66);
        }

        if l >= 131 {
            l -= 131
        } else {
            break;
        }
    }

    reachable
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let map = parse_input(&input);

    let answer = count_all_reachable(&map);
    println!("answer: {answer}");
}
