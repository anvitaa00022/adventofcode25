use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input.txt"; 

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    let mut result = 0;
    let mut last_position = 50;

    for line in reader.lines() {
        let line_content = line?;
    
        let mut chars = line_content.chars();
    
        if let Some(first_char) = chars.next() {    
            let is_left = first_char == 'L';
            let num_string = chars.as_str();
            
            match num_string.trim().parse::<i32>() {
                Ok(num) => {
                    let to_move = if is_left { -1 * num } else { num };
                    let new_position = (last_position + to_move).rem_euclid(100);
                    last_position = new_position;
                    if last_position == 0 {
                        result += 1;
                    }
                },
                Err(_) => {
                    eprintln!("Error: Could not parse number from '{}'", num_string);
                }
            }
        } else {
            println!("Found an empty line");
        }
    }
    println!("final: {}, {}", result, last_position);

    Ok(())
}
