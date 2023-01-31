fn main() {
    let age = std::env::args().nth(1).expect("Expected one argument.");
    println!(
        "You are {} days old.",
        age.parse::<i32>()
            .expect("Expected argument to be a number.")
            * 365
    );
}

#[test]
fn validade_success() {
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("10")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"You are 3650 days old.\n");
}

#[test]
fn validate_empty_year() {
    let output = std::process::Command::new("cargo")
        .arg("run")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let res = b"Expected one argument.";
    assert!(output.stderr.windows(res.len()).any(|window| window == res));
}

#[test]
fn validate_invalid_year() {
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("ten")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let res = b"Expected argument to be a number.";
    assert!(output.stderr.windows(res.len()).any(|window| window == res));
}
