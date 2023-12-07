struct Race {
    time: i32,
    distance: i32,
}

fn distance(hold_time: i32, total_time: i32) -> i32 {
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

fn ways_to_win(race: &Race) -> i32 {
    (1..=race.time)
        .filter(|t| distance(*t, race.time) > race.distance)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let races = vec![
        Race {
            time: 52,
            distance: 426,
        },
        Race {
            time: 94,
            distance: 1374,
        },
        Race {
            time: 75,
            distance: 1279,
        },
        Race {
            time: 94,
            distance: 1216,
        },
    ];

    let answer: i32 = races.iter().map(ways_to_win).product();
    println!("answer: {answer}");
}
