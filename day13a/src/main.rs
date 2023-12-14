use std::fs;

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut lines = input.lines();
    let mut patterns = Vec::new();

    loop {
        let pattern: Vec<_> = lines
            .by_ref()
            .take_while(|l| l != &"")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        if pattern.len() == 0 {
            break;
        }

        patterns.push(pattern);
    }

    patterns
}

fn reflects_at(p: &[char], n: usize) -> bool {
    for i in 0.. {
        if n < 1 + i {
            return true;
        }
        match (p.get(n + i), p.get(n - 1 - i)) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                } else {
                    return false;
                }
            }
            (_, None) => return true,
            (None, _) => return true,
        }
    }
    true
}

#[test]
fn test_reflects_at() {
    fn v(p: &str) -> Vec<char> {
        p.chars().collect::<Vec<_>>()
    }

    assert_eq!(reflects_at(&v(&"#.##..##."), 1), false);
    assert_eq!(reflects_at(&v(&"#.##..##."), 2), false);
    assert_eq!(reflects_at(&v(&"#.##..##."), 3), false);
    assert_eq!(reflects_at(&v(&"#.##..##."), 4), false);
    assert_eq!(reflects_at(&v(&"#.##..##."), 5), true);
    assert_eq!(reflects_at(&v(&"#.##..##."), 6), false);
    assert_eq!(reflects_at(&v(&"#.##..##."), 7), true);
    assert_eq!(reflects_at(&v(&"#.##..##."), 8), false);
}

fn score_pattern(pattern: &Vec<Vec<char>>) -> Option<usize> {
    // check for vertical-line reflection
    let mut candidates: Option<Vec<usize>> = None;
    for i in 0..pattern.len() {
        if candidates == None {
            candidates = Some((1..pattern[i].len()).collect());
        }
        candidates = Some(
            candidates
                .unwrap()
                .iter()
                .filter(|j| reflects_at(&pattern[i], **j))
                .map(|j| *j)
                .collect(),
        );
    }
    if let Some(cs) = candidates {
        if let Some(i) = cs.get(0) {
            if cs.len() == 1 {
                return Some(*i);
            }
        }
    }

    // check for horizontal-line reflection
    let mut candidates: Option<Vec<usize>> = None;
    for i in 0..pattern[0].len() {
        let seq: Vec<_> = (0..pattern.len()).map(|j| pattern[j][i]).collect();
        if candidates == None {
            candidates = Some((1..seq.len()).collect());
        }
        candidates = Some(
            candidates
                .unwrap()
                .iter()
                .filter(|j| reflects_at(&seq, **j))
                .map(|j| *j)
                .collect(),
        );
    }
    if let Some(cs) = candidates {
        if let Some(i) = cs.get(0) {
            if cs.len() == 1 {
                return Some(*i * 100);
            }
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let patterns = parse_input(&input);

    // for p in &patterns {
    //     println!("");
    //     for l in p {
    //         println!("{:?}", l);
    //     }
    // }

    let answer: usize = patterns.iter().flat_map(score_pattern).sum();

    println!("answer: {answer}");
}
