use counter::Counter;
use std::fs;

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: i32,
    t: Type,
    rank: (i32, [i32; 5]),
}

#[derive(Debug)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand<'_> {
    fn new(input_line: &str) -> Hand {
        let mut parts = input_line.split_whitespace();
        let cards = parts.next().expect("no cards");
        let bid = parts
            .next()
            .expect("no bid")
            .parse::<i32>()
            .expect("bid is not a number");
        let t = get_type(cards).expect("weird hand {cards}");
        let rank = get_rank(cards, &t);

        Hand {
            cards,
            bid,
            t,
            rank,
        }
    }
}

fn get_type(cards: &str) -> Option<Type> {
    let mut counts: Vec<_> = cards.chars().collect::<Counter<_>>().most_common_ordered();
    counts.sort_by_key(|c| (c.1, card_rank(c.0)));
    counts.reverse();

    match counts.as_slice() {
        [(_, 5)] => Some(Type::FiveOfAKind),

        [('J', 4), (_, 1)] => Some(Type::FiveOfAKind),
        [(_, 4), ('J', 1)] => Some(Type::FiveOfAKind),
        [(_, 4), (_, 1)] => Some(Type::FourOfAKind),

        [('J', 3), (_, 2)] => Some(Type::FiveOfAKind),
        [(_, 3), ('J', 2)] => Some(Type::FiveOfAKind),
        [(_, 3), (_, 2)] => Some(Type::FullHouse),

        [('J', 3), (_, 1), (_, 1)] => Some(Type::FourOfAKind),
        [(_, 3), (_, 1), ('J', 1)] => Some(Type::FourOfAKind),
        [(_, 3), (_, 1), (_, 1)] => Some(Type::ThreeOfAKind),

        [(_, 2), ('J', 2), (_, 1)] => Some(Type::FourOfAKind),
        [(_, 2), (_, 2), ('J', 1)] => Some(Type::FullHouse),
        [(_, 2), (_, 2), (_, 1)] => Some(Type::TwoPair),

        [('J', 2), (_, 1), (_, 1), (_, 1)] => Some(Type::ThreeOfAKind),
        [(_, 2), (_, 1), (_, 1), ('J', 1)] => Some(Type::ThreeOfAKind),
        [(_, 2), (_, 1), (_, 1), (_, 1)] => Some(Type::OnePair),

        [(_, 1), (_, 1), (_, 1), (_, 1), ('J', 1)] => Some(Type::OnePair),
        [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Some(Type::HighCard),
        _ => None,
    }
}

fn get_rank(cards: &str, t: &Type) -> (i32, [i32; 5]) {
    let type_rank = match t {
        Type::HighCard => 1,
        Type::OnePair => 2,
        Type::TwoPair => 3,
        Type::ThreeOfAKind => 4,
        Type::FullHouse => 5,
        Type::FourOfAKind => 6,
        Type::FiveOfAKind => 7,
    };

    let cards_rank = [
        card_rank(cards.chars().nth(0).unwrap()),
        card_rank(cards.chars().nth(1).unwrap()),
        card_rank(cards.chars().nth(2).unwrap()),
        card_rank(cards.chars().nth(3).unwrap()),
        card_rank(cards.chars().nth(4).unwrap()),
    ];

    (type_rank, cards_rank)
}

fn card_rank(card: char) -> i32 {
    match card {
        'J' => 1,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => c.to_digit(10).unwrap().try_into().unwrap(),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
    hands.sort_by_key(|h| h.rank);

    let answer: i32 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as i32 * h.bid)
        .sum();
    println!("{answer}");
}
