use std::fs;

fn hash(val: &str) -> u32 {
    let mut current = 0;
    for n in val.chars().map(|c| c as u32) {
        current += n;
        current *= 17;
        current %= 256;
    }
    current
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let answer: u32 = input.split(',').map(|s| s.trim()).map(hash).sum();

    println!("answer: {answer}");
}
