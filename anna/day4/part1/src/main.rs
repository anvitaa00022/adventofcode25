use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day4/part1/src/input.txt",
    )
    .expect("Should have been able to read the file");

    let mut result: u64 = 0;

    let grid: Vec<&str> = contents.trim().lines().collect();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1,1), (1, -1), (-1,1), (-1, -1)];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !is_paper_roll(&grid, x, y) {
                continue;
            }
            let mut num_paper_rolls: u32 = 0;
            for &(dx, dy) in &directions {
                let x1 = x as i32 + dx;
                let y1 = y as i32 + dy;
                if x1 < 0 || y1 < 0  || x1 >= grid.len().try_into().unwrap() || y1 >= grid[0].len().try_into().unwrap() || !is_paper_roll(&grid, x1.try_into().unwrap(), y1.try_into().unwrap()) {
                    continue;
                } else {
                    // println!("adding {}, {}", x1, y1);
                    num_paper_rolls += 1;
                }
                // println!("{}, {}, {}", x as i32 + dx, y as i32 + dy, is_paper_roll(&grid, x, y))
            }
            // println!("{}, {}, {}", x, y, num_paper_rolls);
            if num_paper_rolls < 4 {
                result += 1
            }
        }
    }
    println!("{}", result)
}

fn is_paper_roll(grid: &Vec<&str>, i: usize, j: usize) -> bool {
    if let Some(row) = grid.get(i) {
        if let Some(c) = row.chars().nth(j) {
            return c == '@';
        }
    }
    return false;
}
