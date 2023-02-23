use rand::{thread_rng, Rng};

fn main() {
    println!("Get a random maze!");
    println!("Input the number of rows and columns");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let values: Vec<u32> = input
        .split(',')
        .map(|str| {
            str.trim()
                .parse::<u32>()
                .expect("Error at parsing the number")
        })
        .collect();
    println!("{}", draw_maze(values[0], values[1]));
}

fn add_wall(maze: &mut String, col: u32) {
    maze.push_str(&"+---".repeat(col as usize));
    maze.push_str(&"+\n");
}

fn draw_maze(row: u32, col: u32) -> String {
    let mut maze = String::new();
    add_wall(&mut maze, col);
    let mut rnd = thread_rng();
    for i in 1..row * 2 {
        if i % 2 == 0 {
            for _ in 0..col {
                let random: f32 = rnd.gen();
                if random < 0.5 {
                    maze.push_str("+   ");
                } else {
                    maze.push_str("+---");
                }
            }
            maze.push_str("+\n");
        } else {
            maze.push_str("|   ");
            for _ in 1..col {
                let random: f32 = rnd.gen();
                if random < 0.5 {
                    maze.push_str("    ")
                } else {
                    maze.push_str("|   ");
                }
            }
            maze.push_str("|\n");
        }
    }
    add_wall(&mut maze, col);
    maze
}

#[test]
fn test1() {
    println!("{}", draw_maze(4, 6));
}
