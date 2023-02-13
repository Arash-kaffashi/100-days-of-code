use std::io::stdin;

fn main() {
    println!("Input n non-negative integers, representing an elevation map, to compute how much water it can trap after raining.");
    let mut str = String::new();
    stdin().read_line(&mut str).expect("Error at reading line.");
    let numbers: Result<Vec<u32>, _> = str
        .trim()
        .split(',')
        .map(|n| n.trim().parse::<u32>())
        .collect();
    if numbers.is_err() {
        println!("Error at parsing. Try again.")
    } else {
        println!(
            "It can trap {} volumes of water after raining",
            trapping(numbers.unwrap())
        );
    }
}

fn trapping(walls: Vec<u32>) -> u32 {
    let mut pointer_right = walls.len() - 1;
    if pointer_right <= 0 {
        panic!("No elevation map inputed.");
    }
    let mut pointer_left = 0;
    let mut max_right = walls[pointer_right];
    let mut max_left = walls[pointer_left];
    let mut result = 0;

    while pointer_left < pointer_right {
        if max_left <= max_right {
            pointer_left += 1;
            max_left = max_left.max(walls[pointer_left]);
            result += max_left - walls[pointer_left];
        } else {
            pointer_right -= 1;
            max_right = max_right.max(walls[pointer_right]);
            result += max_right - walls[pointer_right];
        }
    }

    result
}

#[test]
fn test1() {
    assert_eq!(trapping(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
}

#[test]
fn test2() {
    assert_eq!(trapping(vec![4, 2, 0, 3, 2, 5]), 9)
}
