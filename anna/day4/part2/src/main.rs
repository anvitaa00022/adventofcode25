use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day4/part1/src/input.txt",
    )
    .expect("Should have been able to read the file");

    let grid: Vec<&str> = contents.trim().lines().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1,1), (1, -1), (-1,1), (-1, -1)];

    let mut paper_rolls = vec![vec![-1i32; cols]; rows];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !is_paper_roll(&grid, x, y) {
                continue;
            }
            let mut num_paper_rolls: i32 = 0;
            for &(dx, dy) in &directions {
                let x1 = x as i32 + dx;
                let y1 = y as i32 + dy;
                if x1 < 0 || y1 < 0 {
                    continue;
                }
                let (x1, y1) = (x1 as usize, y1 as usize);
                if x1 >= rows || y1 >= cols || !is_paper_roll(&grid, x1, y1){
                    continue;
                } else {
                    // println!("adding {}, {}", x1, y1);
                    num_paper_rolls += 1;
                }
                // println!("{}, {}, {}", x as i32 + dx, y as i32 + dy, is_paper_roll(&grid, x, y))
            }
            // println!("{}, {}, {}", x, y, num_paper_rolls);
            paper_rolls[x][y] = num_paper_rolls;
            if num_paper_rolls < 4 {
                queue.push_back((x, y));
                visited.insert((x, y));
            }
        }
    }

    let mut result: u64 = 0;
    while let Some((x, y)) = queue.pop_front() {
        result += 1;

        // decrement paper rolls for all directions
        for &(dx, dy) in &directions {
            let x1 = x as i32 + dx;
            let y1 = y as i32 + dy;
            if x1 < 0 || y1 < 0 {
                continue;
            }
            let (x1, y1) = (x1 as usize, y1 as usize);
            if x1 >= rows || y1 >= cols || !is_paper_roll(&grid, x1, y1){
                continue;
            } else {
                paper_rolls[x1][y1] -= 1;
                if paper_rolls[x1][y1] < 4 {
                    if !visited.contains(&(x1, y1)) {
                        queue.push_back((x1, y1));
                        visited.insert((x1, y1));
                    }
                }
            }
        }
    }
    println!("{}", result);
}

fn is_paper_roll(grid: &Vec<&str>, i: usize, j: usize) -> bool {
    if let Some(row) = grid.get(i) {
        if let Some(c) = row.chars().nth(j) {
            return c == '@';
        }
    }
    return false;
}
