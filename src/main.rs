use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file_path = "/Users/adityaparekh/Advent/1/secret_entrance/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut curr: i32 = 50;
    let mut result: i32 = 0;

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        // Handle potential errors when reading each line
        let line = line_result?;

        let mut left: bool = false;
        if line.starts_with('L') {
            left = true;
        }
        
        let mut num: i32 = 0;
        
        for (index, nums) in line.char_indices() {
            if index == 0 {
                continue;
            }

            let my_int: i32 = nums.to_digit(10).unwrap().try_into().unwrap();
            num = num * 10 + my_int;
        }

        num = num.rem_euclid(100);

        if left {
            curr = curr - num;
        } else {
            curr = curr + num
        }

        if curr < 0 {
            curr += 100;
        } else if curr >= 100 {
            curr -= 100;
        }

        if curr == 0 {
            result += 1;
        }


        //println!("{} {}", curr, num);
    }

    println!("Result: {}", result);

    Ok(())
}