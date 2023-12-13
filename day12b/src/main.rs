use lru::LruCache;
use std::fs;
use std::num::NonZeroUsize;

static FOLDS: usize = 5;

fn parse_record(line: &str) -> (String, Vec<u64>) {
    let mut parts = line.split_whitespace();

    let mut raw_conds_str = parts.next().expect("no conditions").to_string();
    raw_conds_str.push('?');
    let mut conds_str = raw_conds_str.repeat(FOLDS);
    conds_str.pop();
    let conds = conds_str;

    let groups_str = parts.next().expect("no groups");
    let groups = groups_str
        .split(',')
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<_>>()
        .repeat(FOLDS);

    (conds, groups)
}

fn possible_arrangements<'a>(
    cache: &mut LruCache<(String, Vec<u64>), u64>,
    current_run: u64,
    substr: &'a str,
    groups: &'a [u64],
    remain: u64,
) -> u64 {
    if substr.len() == 0 {
        if groups.len() == 0 && current_run == 0 {
            return 1;
        } else if groups.len() == 1 && current_run == groups[0] {
            return 1;
        } else {
            return 0;
        }
    }
    if current_run == 0 && remain < groups.iter().sum() {
        return 0;
    }

    // println!("");
    // println!("remain: {remain}");
    // println!("substr: {substr}");
    // println!("remaining groups: {:?}", groups);

    // if (current_run + remain) < groups.iter().sum() {
    //     // println!("not enough left");
    //     // println!("");
    //     // println!("current_run: {current_run}");
    //     // println!("remain: {remain}");
    //     // println!("substr: {substr}");
    //     // println!("remaining groups: {:?}", groups);
    //     return 0;
    // }

    match substr.chars().next().unwrap() {
        '#' => {
            if groups.len() == 0 || current_run >= groups[0] {
                0
            } else {
                possible_arrangements(cache, current_run + 1, &substr[1..], groups, remain - 1)
            }
        }
        '.' => {
            if current_run == 0 {
                possible_arrangements(cache, 0, &substr[1..], groups, remain)
            } else if groups.len() != 0 && current_run == groups[0] {
                // println!("");
                // println!("substr: {substr}");
                // println!("remaining groups: {:?}", &groups[1..]);

                let substr_copy: String = substr[1..].chars().collect();
                let groups_copy: Vec<_> = groups[1..].iter().map(|e| *e).collect();
                let key = (substr_copy, groups_copy);

                if let Some(n) = cache.get(&key) {
                    *n
                } else {
                    let m = possible_arrangements(cache, 0, &substr[1..], &groups[1..], remain);
                    cache.put(key, m);
                    m
                }
            } else {
                0
            }
        }
        '?' => {
            let rest = &substr[1..];
            let a: String = "#".chars().chain(rest.chars()).collect();
            let b: String = ".".chars().chain(rest.chars()).collect();

            possible_arrangements(cache, current_run, &a, groups, remain)
                + possible_arrangements(cache, current_run, &b, groups, remain - 1)
        }
        _ => panic!(),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let records: Vec<_> = input.lines().map(parse_record).collect();

    // for r in &records {
    //     println!("{:?}", r);
    // }
    let mut cache: LruCache<(String, Vec<u64>), u64> =
        LruCache::new(NonZeroUsize::new(1_000_000_000).unwrap());

    let answer: u64 = records
        .iter()
        .enumerate()
        .map(|(i, (s, groups))| {
            // println!("");
            // println!("{s} {:?}", groups);
            let start = std::time::Instant::now();
            let remain = s.chars().filter(|c| c != &'.').count() as u64;
            let n = possible_arrangements(&mut cache, 0, s, groups, remain);
            let duration = start.elapsed();
            println!("row {i} answer: {n} in {:?}", duration);
            n
        })
        .sum();

    println!("answer: {answer}");
}
