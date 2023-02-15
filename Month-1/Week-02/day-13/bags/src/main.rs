fn main() {
    println!("Hello, world!");
}

fn can_fit(weights: &mut Vec<u32>, bags: u32) -> bool {
    const MAX_WEIGHT: u32 = 10;

    weights.sort();

    let mut n_bags = 1;
    while weights.len() > 1 {
        let mut sum = weights[0];
        let mut remove: Vec<usize> = vec![0];
        for (i, w) in weights.iter().skip(1).enumerate() {
            if sum + w <= MAX_WEIGHT {
                sum += w;
                remove.push(i);
            }
        }
        remove.iter().for_each(|index| {
            weights.remove(*index);
        });
        n_bags += 1;
    }
    n_bags <= bags
}

#[test]
fn test1() {
    assert_eq!(
        can_fit(&mut vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4),
        true
    );
}

#[test]
fn test2() {
    assert_eq!(
        can_fit(&mut vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2], 4),
        false
    );
}
