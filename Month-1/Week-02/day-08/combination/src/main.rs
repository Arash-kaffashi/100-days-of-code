use std::io::stdin;
use std::iter::{once, repeat};

// solution
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/1148716/rust-1-expression-solution/?q=rust&orderBy=hot

fn main() {
    let mut input = String::new();
    println!("Input digits from 2-9 inclusive to get phone letter combinations");
    stdin()
        .read_line(&mut input)
        .expect("Unexpected error at read_line");
    println!("{:?}", permutations(input.trim()))
}

fn permutations(digits: &str) -> Vec<String> {
    digits.chars().fold(
        match digits.is_empty() {
            true => Vec::new(),
            false => vec![String::new()],
        },
        |acc, digit| {
            let letters = match digit {
                '2' => 'a'..='c',
                '3' => 'd'..='f',
                '4' => 'g'..='i',
                '5' => 'j'..='l',
                '6' => 'm'..='o',
                '7' => 'p'..='s',
                '8' => 't'..='v',
                '9' => 'w'..='z',
                _ => panic!(),
            };
            acc.iter()
                .flat_map(|x| {
                    repeat(x)
                        .zip(letters.clone())
                        .map(|(x, y)| x.chars().chain(once(y)).collect::<String>())
                })
                .collect::<Vec<String>>()
        },
    )
}

#[test]
fn test1() {
    assert_eq!(
        permutations("23"),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}
