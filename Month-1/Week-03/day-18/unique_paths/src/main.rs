use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
fn main() {
    let mut m = String::new();
    let mut n = String::new();
    println!("Unique Paths");
    println!("A robot is located at the top-left corner of a m x n grid.\nThe robot can only move either down or right at any point in time.\nThe robot is trying to reach the bottom-right corner of the grid.\nHow many possible unique paths are there?\n");
    print!("Input:\nm = ");
    stdout().flush().unwrap();
    stdin().read_line(&mut m).unwrap();
    print!("n = ");
    stdout().flush().unwrap();
    stdin().read_line(&mut n).unwrap();
    println!(
        "Output: {}",
        unique_paths(
            m.trim().parse().expect("Unable to parse m"),
            n.trim().parse().expect("Unable to parse n")
        )
    );
}

fn factorial(n: u32, start: Option<u32>) -> u32 {
    let start = start.unwrap_or(1);
    if n < 2 {
        return 1;
    }
    (start..=n).product()
}

fn unique_paths(m: u32, n: u32) -> u32 {
    let (a, b) = match m.cmp(&n) {
        Ordering::Equal | Ordering::Less => (m, n),
        Ordering::Greater => (n, m),
    };
    factorial(a + b - 2, Some(b)) / factorial(a - 1, None)
}

#[test]
fn example1() {
    assert_eq!(unique_paths(3, 7), 28);
}
#[test]
fn example2() {
    assert_eq!(unique_paths(3, 2), 3);
}
#[test]
fn example3() {
    assert_eq!(unique_paths(7, 3), 28);
}
#[test]
fn example4() {
    assert_eq!(unique_paths(3, 3), 6);
}
