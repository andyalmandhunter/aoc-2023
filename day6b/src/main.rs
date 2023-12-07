struct Race {
    time: i64,
    distance: i64,
}

fn distance(hold_time: i64, total_time: i64) -> i64 {
    hold_time * (total_time - hold_time)
}

#[test]
fn test_distance() {
    assert_eq!(distance(0, 7), 0);
    assert_eq!(distance(1, 7), 6);
    assert_eq!(distance(2, 7), 10);
    assert_eq!(distance(3, 7), 12);
    assert_eq!(distance(4, 7), 12);
    assert_eq!(distance(5, 7), 10);
    assert_eq!(distance(6, 7), 6);
    assert_eq!(distance(7, 7), 0);
}

fn ways_to_win(race: &Race) -> i64 {
    (1..=race.time)
        .filter(|t| distance(*t, race.time) > race.distance)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let race = Race {
        time: 52947594,
        distance: 426137412791216,
    };

    let answer: i64 = ways_to_win(&race);
    println!("answer: {answer}");
}
