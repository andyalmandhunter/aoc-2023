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

fn smudge_count(p: &[char], n: usize) -> u32 {
    let mut sc = 0;
    for i in 0.. {
        if n < 1 + i {
            break;
        }
        match (p.get(n + i), p.get(n - 1 - i)) {
            (Some(a), Some(b)) => {
                if a != b {
                    sc += 1;
                }
            }
            (_, None) => break,
            (None, _) => break,
        }
    }
    sc
}

#[test]
fn test_smudge_count() {
    fn v(p: &str) -> Vec<char> {
        p.chars().collect::<Vec<_>>()
    }

    assert_eq!(smudge_count(&v(&"#.##..#"), 3), 1);
    assert_eq!(smudge_count(&v(&"..##..#"), 3), 0);

    assert_eq!(smudge_count(&v(&"#..##.."), 1), 1);
    assert_eq!(smudge_count(&v(&"##.##.."), 1), 0);
}

fn score_pattern(pattern: &Vec<Vec<char>>) -> Option<usize> {
    // check for vertical-line reflection with exactly one smudge
    for i in 0..pattern[0].len() {
        let smudges: u32 = pattern.iter().map(|p| smudge_count(p, i)).sum();
        if smudges == 1 {
            println!("vert: {i}");
            return Some(i);
        }
    }

    // check for horizontal-line reflection with exactly one smudge
    for i in 0..pattern.len() {
        let mut smudges = 0;
        for j in 0..pattern[0].len() {
            let seq: Vec<_> = (0..pattern.len()).map(|k| pattern[k][j]).collect();
            smudges += smudge_count(&seq, i);
        }
        if smudges == 1 {
            println!("horiz: {i}");
            return Some(i * 100);
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
