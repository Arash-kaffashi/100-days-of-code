fn main() {
    println!("Hello, world!");
}

fn encrypt(str: &str) -> String {
    str.chars()
        .rev()
        .map(|char| match char {
            'a' => '0',
            'e' => '1',
            'i' | 'o' => '2',
            'u' => '3',
            _ => char,
        })
        .collect::<String>()
        + &"aca"
}

#[test]
fn test() {
    assert_eq!(encrypt("apple"), "1lpp0aca");
    assert_eq!(encrypt("banana"), "0n0n0baca");
    assert_eq!(encrypt("karaca"), "0c0r0kaca");
    assert_eq!(encrypt("burak"), "k0r3baca");
    assert_eq!(encrypt("alpaca"), "0c0pl0aca");
}
