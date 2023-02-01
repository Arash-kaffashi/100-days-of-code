fn main() {}

fn find_nemo(string: &str) -> Option<usize> {
    let result = string.split_whitespace().position(|x| x == "Nemo");

    if result.is_some() {
        println!("I found Nemo at {}!", result.unwrap() + 1);
    } else {
        println!("I can't find Nemo :(");
    }
    result
}

#[test]
fn nemo_at_end() {
    let result = find_nemo("I am finding Nemo !");
    assert_eq!(result, Some(3));
}
#[test]
fn nemo_at_start() {
    let result = find_nemo("Nemo is me");
    assert_eq!(result, Some(0));
}
#[test]
fn nemo_at_middle() {
    assert_eq!(find_nemo("I Nemo am"), Some(1));
}

#[test]
fn nemo_with_something() {
    assert_eq!(find_nemo("Nemo's friend is Dory"), None);
}
#[test]
fn nemo_not_found() {
    assert_eq!(find_nemo("I am finding Dory !"), None);
}
