use std::io::stdin;

fn main() {
    let mut first = String::new();
    let mut second = String::new();
    println!("Valid Anagram: Given two strings s and t, determine if t is an anagram of s");
    println!("Input the first string");
    stdin().read_line(&mut first).unwrap();
    println!("Input the second string");
    stdin().read_line(&mut second).unwrap();
    println!(
        "Is a valid anagram? {}",
        is_anagram(&first.trim().to_lowercase(), &second.trim().to_lowercase())
    );
}

fn is_anagram(s: &str, t: &str) -> bool {
    let [mut first, mut second] = [s, t].map(|str| str.chars().collect::<Vec<char>>());
    first.sort();
    second.sort();
    first.eq(&second)
}

#[test]
fn test() {
    assert!(is_anagram("anagram", "nagaram"));
}

#[test]
fn test2() {
    assert!(!is_anagram("rat", "car"));
}
