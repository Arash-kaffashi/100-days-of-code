use std::io::{self, Write};

fn main() {
    println!("Nim Game!");
    println!("Given n, the number of stones in the heap, return true if you can win the game assuming both you and your friend play optimally, otherwise return false.");
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let res = nim_game(s.trim().parse::<u32>().expect("Error at parsing input."));
    println!("Output: {res}");
}

fn nim_game(n: u32) -> bool {
    n % 4 != 0
}

#[test]
fn test() {
    assert!(!nim_game(4));
    assert!(nim_game(1));
    assert!(nim_game(2));
}
