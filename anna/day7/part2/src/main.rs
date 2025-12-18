use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day7/test.txt")
        .expect("Should have been able to read the file");
    
    let lines: Vec<&str> = contents.trim().lines().collect();
    let rows: usize = lines.len();
    let cols: usize = lines[0].len();
    let mut memo: Vec<Vec<i64>> = vec![vec![-1; cols]; rows];

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            grid[row][col] = lines[row].chars().nth(col).unwrap();
        }
    }

    let mut start_ind = 0;
    while start_ind < cols {
        if lines[0].chars().nth(start_ind).unwrap() == 'S' {
            break;
        }
        start_ind += 1;
    }

    println!("{}", start_ind);
    println!("{}", recurse(1, start_ind, &grid, &mut memo));

    Ok(())
}

fn recurse(row: usize, col: usize, grid: &Vec<Vec<char>>, memo: &mut Vec<Vec<i64>>) -> i64 {
    if row >= grid.len() || col >= grid[0].len() {
        return 0;
    }
    let mut result: i64 = 0;
    if memo[row][col] > -1 {
        return memo[row][col];
    } else if row == grid.len()-1 {
        memo[row][col] = 1;
        return 1;
    }
    let curr = grid[row][col];
    if curr == '.' {
        result = recurse(row+1, col, grid, memo);
    } else if curr == '^' {
        result = recurse(row, col-1, grid, memo) + recurse(row, col+1, grid, memo);
    }
    memo[row][col] = result;
    return result;
}
