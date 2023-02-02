use std::io::stdin;

fn main() {
    println!("Input a number and get it or the next prime");
    let mut str = String::new();
    stdin()
        .read_line(&mut str)
        .expect("Error reading the input.");
    let inpt = str.trim().parse::<i32>().expect("Input must be a number");
    let res = next_prime(inpt);
    println!(
        "{} {}",
        if res == inpt {
            "It is already a prime number:"
        } else {
            "The next prime number is:"
        },
        res
    )
}

fn is_prime(num: i32) -> bool {
    if num > 1 && num <= 3 {
        return true;
    }
    if num < 2 || num % 2 == 0 {
        return false;
    }
    for i in (3..=(num as f64).sqrt() as i32).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn next_prime(mut num: i32) -> i32 {
    while !is_prime(num) {
        num += 1;
    }
    num
}

#[test]
fn test1() {
    assert_eq!(next_prime(12), 13)
}
#[test]
fn test2() {
    assert_eq!(next_prime(24), 29)
}

#[test]
fn test3() {
    assert_eq!(next_prime(11), 11)
}
#[test]
fn test4() {
    assert_eq!(next_prime(16), 17)
}
