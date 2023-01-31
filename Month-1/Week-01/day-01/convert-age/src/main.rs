fn main() {
    let age = std::env::args().nth(1).expect("Expected one argument.");
    println!(
        "You are {} days old.",
        age.parse::<i32>()
            .expect("Expected argument to be a number.")
            * 365
    );
}
