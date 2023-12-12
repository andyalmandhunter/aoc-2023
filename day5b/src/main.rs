use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn get(&self, key: i64) -> i64 {
        for e in &self.entries {
            if let Some(v) = e.get(key) {
                return v;
            }
        }
        key
    }
}

#[test]
fn test_map_get() {
    let m = Map {
        entries: vec![Entry::from_str("50 98 2"), Entry::from_str("52 50 48")],
    };

    assert_eq!(m.get(79), 81);
    assert_eq!(m.get(100), 100);
}

#[derive(Debug)]
struct Entry {
    key_start: i64,
    value_start: i64,
    range_len: i64,
}

impl Entry {
    fn new(key_start: i64, value_start: i64, range_len: i64) -> Self {
        Self {
            key_start,
            value_start,
            range_len,
        }
    }

    fn from_str(raw: &str) -> Self {
        let parts: Vec<_> = raw
            .split_whitespace()
            .flat_map(|s| s.parse::<i64>())
            .collect();
        Self::new(parts[1], parts[0], parts[2])
    }

    fn get(&self, key: i64) -> Option<i64> {
        if key < self.key_start || key >= self.key_start + self.range_len {
            None
        } else {
            Some((key - self.key_start) + self.value_start)
        }
    }
}

#[test]
fn test_entry_get() {
    let e = Entry::new(50, 98, 2);
    assert_eq!(e.get(49), None);
    assert_eq!(e.get(50), Some(98));
    assert_eq!(e.get(51), Some(99));
    assert_eq!(e.get(52), None);
}

fn next_map<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Map {
    let entries: Vec<_> = lines
        .skip_while(|l| !l.ends_with("map:"))
        .skip(1)
        .take_while(|l| l != &"")
        .map(Entry::from_str)
        .collect();

    Map { entries }
}

fn seeds_range(start: i64, n: i64) -> Vec<i64> {
    (start..(start + n)).collect()
}

fn get_locations(input: &str) -> i64 {
    let mut lines = input.lines();

    let seeds_line = lines.next().expect("no seeds line");
    let seeds_raw = seeds_line.split(":").skip(1).next().expect("no seeds");
    let seeds_ranges: Vec<_> = seeds_raw
        .split_whitespace()
        .flat_map(|s| s.parse::<i64>())
        .tuples::<(_, _)>()
        .collect();
    let seeds: Vec<_> = seeds_ranges
        .iter()
        .flat_map(|(start, n)| seeds_range(*start, *n))
        .collect();

    println!("number of seeds: {}", seeds.len());

    let seed_to_soil_map = next_map(&mut lines);
    let soil_to_fertilizer_map = next_map(&mut lines);
    let fertilizer_to_water_map = next_map(&mut lines);
    let water_to_light_map = next_map(&mut lines);
    let light_to_temperature_map = next_map(&mut lines);
    let temperature_to_humidity_map = next_map(&mut lines);
    let humidity_to_location_map = next_map(&mut lines);

    seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil_map.get(*seed);
            let fertilizer = soil_to_fertilizer_map.get(soil);
            let water = fertilizer_to_water_map.get(fertilizer);
            let light = water_to_light_map.get(water);
            let temperature = light_to_temperature_map.get(light);
            let humidity = temperature_to_humidity_map.get(temperature);
            humidity_to_location_map.get(humidity)
        })
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer = get_locations(&input);

    println!("answer: {answer}");
}
