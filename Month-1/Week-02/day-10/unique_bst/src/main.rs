use std::io::stdin;

fn main() {
    println!("Given n, how many structurally unique BST's (binary search trees) that store values 1 ... n?");
    let mut str = String::new();
    stdin().read_line(&mut str).expect("Error at reading line.");
    let number = str
        .trim()
        .parse()
        .expect("Error at parsing. Try again. Input a valid number. i <= n <= 19");

    println!(
        "Given n = {}, there are a total of {} unique BST's",
        number,
        get_unique_bst(number)
    );
}

// https://en.wikipedia.org/wiki/Catalan_number
fn get_unique_bst(n: i32) -> i32 {
    let n = n as i128;
    let a = (2..=n).fold(1, |prod, k| prod * (n + k));
    let b = (2..=n).fold(1, |prod, k| prod * k);
    (a / b) as i32
}

#[test]
fn test() {
    assert_eq!(
        vec![
            1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845,
            35357670, 129644790, 477638700, 1767263190
        ],
        (1..=19).map(|n| get_unique_bst(n)).collect::<Vec<i32>>()
    );
}
