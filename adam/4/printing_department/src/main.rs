use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let _ = part_one();
    let _ = part_two();

    Ok(())
}

fn part_one() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/adam/4/printing_department/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i8>> = Vec::new();

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let mut row: Vec<i8> = Vec::new();

        for c in line_result?.chars() {
            if c == '@' {
                row.push(1);
            } else {
                row.push(0);
            }
        }

        grid.push(row);
    }

    let mut num_rolls: i32 = 0;
    let dirs: Vec<(isize,isize)> = vec![(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                let mut curr: i8 = 0;
                for (dj, di) in dirs.iter() {
                    match i.checked_add_signed(*di) {
                        Some(result) => {
                            match j.checked_add_signed(*dj) {
                                Some(result2) => {
                                    if result < grid.len() && result2 < grid[i].len() && grid[result][result2] == 1 {
                                        curr += 1;
                                    }
                                }
                                _ => (),
                            }

                        }
                        _ => (),
                    }
                }

                if curr < 4 {
                    num_rolls += 1;
                }
            }
        }
    }

    println!("num_rolls: {}", num_rolls);

    Ok(())
}

fn part_two() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/adam/4/printing_department/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i8>> = Vec::new();
    let mut grid2: Vec<Vec<i8>> = Vec::new();

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let mut row: Vec<i8> = Vec::new();
        let mut row2: Vec<i8> = Vec::new();

        for c in line_result?.chars() {
            if c == '@' {
                row.push(1);
            } else {
                row.push(0);
            }

            row2.push(0);
        }

        grid.push(row);
        grid2.push(row2);
    }

    let dirs: Vec<(isize,isize)> = vec![(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];
    let mut can_help: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                let mut curr: i8 = 0;
                for (dj, di) in dirs.iter() {
                    match i.checked_add_signed(*di) {
                        Some(result) => {
                            match j.checked_add_signed(*dj) {
                                Some(result2) => {
                                    if result < grid.len() && result2 < grid[i].len() && grid[result][result2] == 1 {
                                        curr += 1;
                                    }
                                }
                                _ => (),
                            }

                        }
                        _ => (),
                    }
                }
                grid2[i][j] = curr;
            }
        }
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 && grid2[i][j] < 4 {
                queue.push_back((i,j));
                visited.insert((i,j));
            }
        }
    }

    while queue.len() > 0 {
        let elem = queue.pop_front().unwrap();
        let i: usize = elem.0;
        let j: usize = elem.1;

        can_help += 1;

        for (dj, di) in dirs.iter() {
            match i.checked_add_signed(*di) {
                Some(result) => {
                    match j.checked_add_signed(*dj) {
                        Some(result2) => {
                            if result < grid.len() && result2 < grid[i].len() && grid[result][result2] == 1 {
                                grid2[result][result2] -= 1;

                                if !visited.contains(&(result, result2)) && grid2[result][result2] < 4 {
                                    queue.push_back((result, result2));
                                    visited.insert((result, result2));
                                }
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    println!("can_help: {}", can_help);

    Ok(())
}