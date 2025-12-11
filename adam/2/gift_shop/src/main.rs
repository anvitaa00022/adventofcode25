use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let _ = part_one();
    let _ = part_two();

    Ok(())
}

fn part_one() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/2/gift_shop/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        // Handle potential errors when reading each line
        let line = line_result?;

        let parts: Vec<&str> = line.split(',').collect();

        let mut total_sum: u64 = 0;

        for item in parts.iter() {
            let parts1: Vec<&str> = item.split('-').collect();

            let mut x: u64 = parts1[0].parse().expect("Error");
            let y: u64 = parts1[1].parse().expect("Error");

            while x <= y {
                let len_x: u32 = x.to_string().chars().count().try_into().unwrap();
                let base_int: u64 = 10;

                if  len_x % 2== 1 {
                    x += 1;
                    continue;
                }

                if x % base_int.pow(len_x/2) == x/base_int.pow(len_x/2) {
                    total_sum += x;
                }

                x += 1;
            }
        }
        println!("{}", total_sum);
    }

    Ok(())
}

fn part_two() -> Result<(), io::Error> {
    let file_path = "/Users/adityaparekh/Advent/2/gift_shop/src/input2.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        // Handle potential errors when reading each line
        let line = line_result?;

        let parts: Vec<&str> = line.split(',').collect();

        let mut total_sum: u64 = 0;

        for item in parts.iter() {
            let parts1: Vec<&str> = item.split('-').collect();

            let x: u64 = parts1[0].parse().expect("Error");
            let y: u64 = parts1[1].parse().expect("Error");

            for j in x..=y {
                let len_x: u32 = j.to_string().chars().count().try_into().unwrap();
                for i in 1..=len_x/2 {
                    if len_x % i == 0 {
                        let first_chars: String = j.to_string().chars().take(i.try_into().unwrap()).collect();
                        let repeated_str = first_chars.repeat((len_x/i).try_into().unwrap());

                        let gen_num: u64 = repeated_str.parse().expect("Error");

                        if j == gen_num {
                            total_sum += gen_num;
                            break;
                        }

                    }
                }
            }
            
        }
        println!("{}", total_sum);
    }

    Ok(())
}

