fn main() {}

fn get_skewers(grill: [&str; 5]) -> [i32; 2] {
    grill
        .iter()
        .fold([0, 0], |[mut veg, mut non_veg], &skewer| {
            if skewer.contains("x") {
                non_veg += 1;
            } else if skewer.contains("o") {
                veg += 1;
            }

            [veg, non_veg]
        })
}

#[test]
fn first_ex() {
    assert_eq!(
        get_skewers([
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ]),
        [1, 4]
    )
}
#[test]
fn second_ex() {
    assert_eq!(
        get_skewers([
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ]),
        [2, 3]
    )
}

#[test]
fn third_ex() {
    assert_eq!(
        get_skewers([
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ]),
        [3, 2]
    )
}
#[test]
fn empty_grill() {
    assert_eq!(get_skewers(["", "", "", "", ""]), [0, 0])
}
