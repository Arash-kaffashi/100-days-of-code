fn main() {
    println!("Input a string containing only digits to get all possible valid IP addresses");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let result = ip(&input.trim());
    if result.len() == 0 {
        println!("No valid IP address found.")
    }
    println!("Output: {:?}", result);
}

fn is_seg_valid(str: &str) -> bool {
    if str.starts_with('0') && str.len() > 1 {
        return false;
    }
    str.parse::<u8>().is_ok()
}

fn ip(str: &str) -> Vec<String> {
    let mut result = vec![];
    let length = str.len();

    if length > 12 || length < 4 {
        return result;
    }

    for i in 1..=3 {
        for j in (i + 1)..=(i + 3) {
            for k in (j + 1)..=(j + 3) {
                if k < length {
                    let seg_1 = &str[..i];
                    let seg_2 = &str[i..j];
                    let seg_3 = &str[j..k];
                    let seg_4 = &str[k..];

                    if [seg_1, seg_2, seg_3, seg_4]
                        .iter()
                        .all(|&seg| is_seg_valid(seg))
                    {
                        result.push(format!("{}.{}.{}.{}", seg_1, seg_2, seg_3, seg_4));
                    }
                }
            }
        }
    }
    result
}

#[test]
fn test1() {
    assert_eq!(ip("25525511135"), vec!["255.255.11.135", "255.255.111.35"])
}
#[test]
fn test2() {
    assert_eq!(ip("0000"), vec!["0.0.0.0"]);
}
#[test]
fn test3() {
    assert_eq!(ip("1111"), vec!["1.1.1.1"]);
}
#[test]
fn test4() {
    assert_eq!(ip("010010"), vec!["0.10.0.10", "0.100.1.0"]);
}
#[test]
fn test5() {
    assert_eq!(
        ip("101023"),
        vec![
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3"
        ]
    )
}
