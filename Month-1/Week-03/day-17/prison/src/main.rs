use std::io::{stdin, stdout, Write};

fn main() {
    println!("Prision Break");
    println!("A prison can be represented as an array of cells. Each cell contains exactly one prisoner. A 1 represents an unlocked cell and a 0 represents a locked cell.");
    print!("Input: ");
    stdout().flush().unwrap();
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();
    let res: Vec<u32> = str
        .split(',')
        .map(|str_n| {
            str_n
                .trim()
                .parse::<u32>()
                .expect("Error at parsing a number.")
        })
        .collect();
    println!("You freed {} prisoners", freed_prisoners(&res));
}

fn freed_prisoners(cells: &[u32]) -> u32 {
    if cells.len() < 1 || cells[0] == 0 {
        return 0;
    }
    let mut result = 0;
    let mut invert = false;
    for cell in cells {
        match (cell, invert) {
            (0, true) | (1, false) => {
                invert = !invert;
                result += 1;
            }
            _ => continue,
        }
    }
    result
}

#[test]
fn ex1() {
    assert_eq!(freed_prisoners(&[1, 1, 0, 0, 0, 1, 0]), 4);
}

#[test]
fn ex2() {
    assert_eq!(freed_prisoners(&[1, 1, 1]), 1);
}

#[test]
fn ex3() {
    assert_eq!(freed_prisoners(&[0, 0, 0]), 0);
}
#[test]
fn ex4() {
    assert_eq!(freed_prisoners(&[0, 1, 1, 1]), 0);
}
