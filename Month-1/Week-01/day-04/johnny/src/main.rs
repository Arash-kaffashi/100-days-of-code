use std::num::ParseIntError;

fn main() {
    println!("Input the progress of the runs: ");
    let mut raw_runs = String::new();
    std::io::stdin()
        .read_line(&mut raw_runs)
        .expect("Failed to read line");

    if let Some(result) = progress_days(&raw_runs) {
        println!("The number of progress days is: {}", result);
    }
}

fn parse_runs(raw_runs: &str) -> Result<Vec<i32>, ParseIntError> {
    let raw_runs = raw_runs.trim();
    if raw_runs.contains(", ") {
        raw_runs.split(", ").map(|s| s.parse::<i32>()).collect()
    } else {
        raw_runs
            .split_terminator(&[' ', ','][..])
            .map(|s| s.parse::<i32>())
            .collect()
    }
}

fn progress_days(raw_runs: &str) -> Option<i32> {
    let parsed_runs = parse_runs(raw_runs);
    if parsed_runs.is_err() {
        println!("Invalid input! {:?}", parsed_runs);
        return None;
    }

    if parsed_runs.as_ref().unwrap().len() < 2 {
        println!("Insuficient input! \"{:?}\"", parsed_runs.unwrap());
        return None;
    }

    let runs = parsed_runs.unwrap();
    let mut count = 0;
    for i in 0..runs.len() - 1 {
        if runs[i] < runs[i + 1] {
            count += 1;
        }
    }
    Some(count)
}

#[test]
fn test1() {
    assert_eq!(progress_days("3, 4, 1, 2"), Some(2));
}
#[test]
fn test2() {
    assert_eq!(progress_days("10, 11, 12, 9, 10"), Some(3));
}
#[test]
fn test3() {
    assert_eq!(progress_days("6, 5, 4, 3, 2, 9"), Some(1));
}
#[test]
fn test4() {
    assert_eq!(progress_days("9, 9"), Some(0));
}
