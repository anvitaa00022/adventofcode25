use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day7/test.txt")
        .expect("Should have been able to read the file");
    
    let lines: Vec<&str> = contents.trim().lines().collect();
    let rows: usize = lines.len();
    let cols: usize = lines[0].len();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            grid[row][col] = lines[row].chars().nth(col).unwrap();
        }
    }
    
    let mut splits = 0;
    for row in 1..rows {
        for col in 0..cols {
            let curr = grid[row][col];
            let prev = grid[row-1][col];
            if prev == '.' {
                continue;
            } else if prev == 'S' || prev == '|' {
                if curr == '^' {
                    splits += 1;
                    grid[row][col-1] = '|';
                    if col+1 < cols {
                        grid[row][col+1] = '|';
                    }
                } else if curr == '.' {
                    grid[row][col] = '|';
                }
            }
        }
    }
    print_grid(grid);
    println!("{}", splits);
    Ok(())
}

fn print_grid(grid: Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
    }
}