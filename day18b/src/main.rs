use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::{Match, Regex};
use std::fs;

#[derive(Debug)]
enum Dir {
    U,
    D,
    R,
    L,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    length: i64,
}

fn number_from_match(m: Match) -> Option<i64> {
    i64::from_str_radix(m.as_str(), 16).ok()
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .lines()
        .flat_map(|l| -> Option<Step> {
            static PATTERN: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"([UDRL]) (\d+) \(#(.{5})(.{1})\)").unwrap());

            let captures = PATTERN.captures(l)?;
            let dir = captures
                .get(4)
                .and_then(number_from_match)
                .and_then(|n| match n {
                    3 => Some(Dir::U),
                    1 => Some(Dir::D),
                    0 => Some(Dir::R),
                    2 => Some(Dir::L),
                    _ => None,
                })?;
            let length = captures.get(3).and_then(number_from_match)?;

            Some(Step { dir, length })
        })
        .collect()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Or {
    TL,
    TR,
    BL,
    BR,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Corner {
    i: i64,
    j: i64,
    orientation: Or,
}

fn get_corners(steps: &[Step]) -> Option<Vec<Corner>> {
    let mut corners: Vec<Corner> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    let mut d = &steps.last()?.dir;

    for s in steps {
        let orientation = match (d, &s.dir) {
            (Dir::U, Dir::R) | (Dir::L, Dir::D) => Or::TL,
            (Dir::R, Dir::D) | (Dir::U, Dir::L) => Or::TR,
            (Dir::D, Dir::L) | (Dir::R, Dir::U) => Or::BR,
            (Dir::L, Dir::U) | (Dir::D, Dir::R) => Or::BL,
            _ => return None,
        };
        corners.push(Corner { i, j, orientation });

        match s.dir {
            Dir::U => i -= s.length,
            Dir::D => i += s.length,
            Dir::R => j += s.length,
            Dir::L => j -= s.length,
        }
        d = &s.dir;
    }

    corners.sort();

    Some(corners)
}

fn get_area(corners: &[Corner]) -> Option<i64> {
    let mut bounds: Vec<i64> = vec![];
    let mut last_i_opt: Option<i64> = None;

    let mut area = 0;
    for (i, group) in &corners.into_iter().group_by(|Corner { i, .. }| i) {
        // 1. add area from previous bounds, if any
        if let Some(last_i) = last_i_opt {
            let height = i - last_i;
            let width: i64 = bounds
                .iter()
                .chunks(2)
                .into_iter()
                .flat_map(|mut chunk| {
                    let left = chunk.next()?;
                    let right = chunk.next()?;
                    Some(right - left + 1)
                })
                .sum();
            // println!("\nadding width {width} height {height}");
            area += (width) * (height - 1);
        }

        // 2. add area from this row
        let row: Vec<_> = group.collect();

        // println!("old bounds: {:?}", bounds);
        // println!("corners: {:?}", row);

        let to_remove: Vec<i64> = row
            .iter()
            .filter(|Corner { orientation, .. }| orientation == &Or::BL || orientation == &Or::BR)
            .map(|Corner { j, .. }| *j)
            .collect();
        let to_add: Vec<i64> = row
            .iter()
            .filter(|Corner { orientation, .. }| orientation == &Or::TL || orientation == &Or::TR)
            .map(|Corner { j, .. }| *j)
            .collect();
        let mut new_bounds: Vec<_> = bounds
            .iter()
            .filter(|b| !to_remove.contains(b))
            .chain(to_add.iter())
            .map(|b| *b)
            .collect();
        new_bounds.sort();

        let merged_bounds = get_merged_bounds(&bounds, &new_bounds)?;
        area += merged_bounds.iter().map(|(a, b)| b - a + 1).sum::<i64>();

        // 3. update bounds and set last_i
        bounds = new_bounds;
        // println!("new bounds {:?}", bounds);

        last_i_opt = Some(*i);
    }

    Some(area)
}

fn bands(bounds: &[i64]) -> Vec<(i64, i64)> {
    bounds
        .iter()
        .chunks(2)
        .into_iter()
        .flat_map(|mut c| {
            let l = c.next()?;
            let r = c.next()?;
            Some((*l, *r))
        })
        .collect()
}

fn get_merged_bounds(a_bounds: &[i64], b_bounds: &[i64]) -> Option<Vec<(i64, i64)>> {
    let mut merged_bounds = bands(a_bounds);
    let b_bands = bands(b_bounds);

    for b in b_bands {
        // collect all items from merged_bounds that overlap at all with b
        //
        // overlap means a.1 >= b.0 and b.1 >= a.0
        let b_vec = vec![b];
        let overlap: Vec<_> = merged_bounds
            .iter()
            .filter(|a| a.1 >= b.0 && b.1 >= a.0)
            .chain(b_vec.iter())
            .collect();

        // remove them from merged_bounds
        let not_overlap: Vec<_> = merged_bounds
            .iter()
            .filter(|a| a.1 < b.0 || b.1 < a.0)
            .collect();

        // add a new item with min left and max right from the collected set
        let l = *overlap.iter().map(|(x, _)| x).min()?;
        let r = *overlap.iter().map(|(_, x)| x).max()?;
        merged_bounds = not_overlap
            .iter()
            .map(|x| *x)
            .chain([(l, r)].iter())
            .map(|x| *x)
            .collect();
    }

    Some(merged_bounds)
}

#[test]
fn test_get_merged_bounds() {
    assert_eq!(get_merged_bounds(&[1, 4], &[3, 6]), Some(vec![(1, 6)]));
    assert_eq!(get_merged_bounds(&[3, 6], &[1, 4]), Some(vec![(1, 6)]));
    assert_eq!(get_merged_bounds(&[1, 6], &[3, 4]), Some(vec![(1, 6)]));
    assert_eq!(
        get_merged_bounds(&[1, 4], &[5, 6]),
        Some(vec![(1, 4), (5, 6)])
    );
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let steps = parse_input(&input);
    let corners = get_corners(&steps).expect("unable to trace path");
    let answer = get_area(&corners).expect("unable to get area");

    println!("answer: {answer}");
}
