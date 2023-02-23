use std::str;

fn main() {
    println!("Marcio Mello's Challenge");
    println!("Enter the deforested area (in km²):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let area = input
        .trim()
        .parse::<f32>()
        .expect(&format!("Error at parsing the input: {}", &input));
    println!(
        "Area covers {} football fields",
        add_dot(to_football_fields(area))
    );
}

fn to_football_fields(area: f32) -> u32 {
    (area / 0.007140).ceil() as u32
}

fn add_dot(n: u32) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".")
}
