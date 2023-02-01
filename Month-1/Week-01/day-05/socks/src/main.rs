use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Input your socks: ");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Error reading the line");
    println!("\nYou have {} sock pairs", sock_pairs(&buffer));
}

fn sock_pairs(str: &str) -> i32 {
    let mut pairs = HashMap::new();
    for char in str.chars() {
        pairs.entry(char).and_modify(|c| *c += 0.5).or_insert(0.5);
    }
    pairs.values().map(|&value| value as i32).sum()
}

#[test]
fn test1() {
    assert_eq!(sock_pairs("AA"), 1)
}
#[test]
fn test2() {
    assert_eq!(sock_pairs("ABABC"), 2)
}
#[test]
fn test3() {
    assert_eq!(sock_pairs("CABBACCC"), 4)
}
#[test]
fn empty() {
    assert_eq!(sock_pairs(""), 0)
}
