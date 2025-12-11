use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let _ = part_one();
    let _ = part_two();

    Ok(())
}

fn part_one() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/3/lobby/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut total_sum: u32 = 0;

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        // Handle potential errors when reading each line
        let line = line_result?;

        let mut largest_so_far: u32 = 0;
        let mut largest_num_so_far: u32 = 0;

        for voltage in line.chars().rev() {
            let v : u32 = voltage.to_digit(10).unwrap();
            
            if largest_num_so_far != 0 && v*10+largest_num_so_far > largest_so_far {
                largest_so_far = v*10+largest_num_so_far;
            }

            if v > largest_num_so_far {
                largest_num_so_far = v;
            }
        }
        
        total_sum += largest_so_far;
    }

    println!("total sum {}", total_sum);
    Ok(())
}

fn part_two() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/3/lobby/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut total_sum: usize = 0;

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        // Handle potential errors when reading each line
        let line = line_result?;

        let mut outer: Vec<Vec<usize>> = Vec::with_capacity(9);

        for _ in 0..9 {
            outer.push(Vec::new());
        }

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut curr_pos: usize = 0;
        let mut largest_so_far: usize = 0;

        while i < 12 {
            while j <= line.len() - (12-i) {
                let v : usize = line.chars().nth(j).expect("REASON").to_digit(10).unwrap().try_into().unwrap();
                outer[v-1].push(j); //I'm pushing to the back (which is efficient)
                j += 1;
            }

            let mut start: usize = 9;

            while start >= 1 {
                while outer[start-1].len() > 0 && outer[start-1][0] < curr_pos {
                    outer[start-1].remove(0); // But i'm removing from the front.
                } //This can be fixed by using a Deque.

                if outer[start-1].len() > 0 {
                    let y: usize = outer[start-1].remove(0);
                    if y >= curr_pos {
                        curr_pos = y+1;
                        largest_so_far = largest_so_far*10 + start;
                        break;
                    }
                }

                start -= 1;
            }

            i += 1;
        }
        
        total_sum += largest_so_far;
    }

    println!("total sum {}", total_sum);

    Ok(())
}