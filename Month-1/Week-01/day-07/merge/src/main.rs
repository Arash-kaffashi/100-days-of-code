use std::{io::stdin, num::ParseIntError};

fn main() {
    println!("Input two arrays to be merged and sorted.");
    println!("Input the first one");
    let mut first = String::new();
    stdin().read_line(&mut first).expect("Error at read_line");
    println!("Input the second one");
    let mut second = String::new();
    stdin().read_line(&mut second).expect("Error at read_line");
    let res1 = get_vec_from_str(&first);
    let res2 = get_vec_from_str(&second);

    if res1.is_err() || res2.is_err() {
        println!("res1: {:?}", res1.unwrap());
        println!("res2: {:?}", res2.unwrap());
        panic!("Error");
    } else {
        let mut nums1 = res1.unwrap();
        let nums2 = res2.unwrap();
        unsafe {
            nums1.set_len(nums1.len() + nums2.len());
        }
        println!("Merged and Sorted: {:?}", merge_array(nums1, nums2));
    }
}

fn get_vec_from_str(str: &str) -> Result<Vec<i32>, ParseIntError> {
    str.trim()
        .split_terminator(&[',', ' '][..])
        .map(|num| num.parse::<i32>())
        .collect()
}

fn merge_array(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let skip = nums1.len() - nums2.len();
    for index in 0..nums2.len() {
        if let Some(elem) = nums1.get_mut(skip + index) {
            *elem = nums2[index];
        }
    }
    nums1.sort();
    nums1
}

#[test]
fn test1() {
    assert_eq!(
        merge_array(vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]),
        vec![1, 2, 2, 3, 5, 6]
    );
}
