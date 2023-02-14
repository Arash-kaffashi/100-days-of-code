use std::{cmp::Ordering, ops::Div};

fn main() {
    println!("Input an array of integers to get the landscape");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let parsed: Result<Vec<i32>, _> = input
        .trim()
        .split(',')
        .map(|n| n.trim().parse::<i32>())
        .collect();
    if parsed.is_err() {
        panic!("Invalid input! Try again!");
    }
    println!("{}", check_landscape(&parsed.unwrap()));
}

fn is_sorted(pos: usize, slice: &[i32], ord: Ordering) -> bool {
    if pos == 0 || pos == slice.len() - 1 {
        return false;
    };
    slice[..=pos].windows(2).all(|w| {
        let cmp = w[0].cmp(&w[1]);
        cmp.is_eq() || cmp == ord
    }) && slice[pos..].windows(2).all(|w| {
        let cmp = w[1].cmp(&w[0]);
        cmp.is_eq() || cmp == ord
    })
}

fn check_landscape(array: &[i32]) -> &str {
    let length = array.len();

    // guard
    if length < 3 {
        return "neither";
    }

    let mut middle = length.div(2);
    if length % 2 == 0 {
        if array[middle] < array[middle - 1] {
            middle -= 1
        }
    };

    if is_sorted(middle, &array, Ordering::Less) {
        return "mountain";
    }

    let pos = array
        .iter()
        .enumerate()
        .min_by(|(_, value1), (_, value2)| value1.cmp(value2))
        .unwrap()
        .0;

    if is_sorted(pos, &array, Ordering::Greater) {
        return "valley";
    }

    "neither"
}

#[test]
fn mountain() {
    assert_eq!(check_landscape(&[1, 3, 5, 4, 3, 2]), "mountain");
    assert_eq!(check_landscape(&[-1, 0, -1]), "mountain");
    assert_eq!(check_landscape(&[-1, -1, 0, -1, -1]), "mountain");
    assert_eq!(check_landscape(&[3, 4, 5, 4, 3]), "mountain");
}

#[test]
fn valley() {
    assert_eq!(check_landscape(&[10, 9, 8, 7, 2, 3, 4, 5]), "valley");
    assert_eq!(check_landscape(&[350, 100, 200, 400, 700]), "valley");
    assert_eq!(check_landscape(&[9, 7, 3, 1, 2, 4]), "valley");
    assert_eq!(check_landscape(&[9, 8, 9]), "valley");
}

#[test]
fn neither() {
    assert_eq!(check_landscape(&[1, 2, 3, 2, 4, 1]), "neither");
    assert_eq!(check_landscape(&[5, 4, 3, 2, 1]), "neither");
    assert_eq!(check_landscape(&[0, -1, -1, 0, -1, -1]), "neither");
    assert_eq!(check_landscape(&[9, 8, 9, 8]), "neither");
}
