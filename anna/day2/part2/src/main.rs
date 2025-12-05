use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day2/part1/src/input.txt")
    .expect("Should have been able to read the file");
    let test: &str = "1212121212";
    println!("{}", recurse_verify(test));
    let parts: Vec<&str> = contents.trim().split(',').collect();
    let mut result: i64 = 0;
    for range in &parts {
        if let Some((left, right)) = range.split_once('-') {
            let low: i64 = left.parse()?;
            let high: i64 = right.parse()?;
            for i in (low..=high).step_by(1) {
                let num_string: &str = &i.to_string();
                result += if is_invalid_id(num_string) { i } else { 0 };
                //println!("{}, {}", low, high, pwd);
            }
            //println!("result: {}", result)
        } else {
            println!("Invalid range: {}", range);
        }
    }
    println!("{}", result);
    Ok(())
    }


fn is_invalid_id(password: &str) -> bool {
    let length = password.len();
    if length <= 1 {
        return false;
    }
    for pattern_len in 1..=(length / 2) {
        if length % pattern_len == 0 {
            let pattern = &password[0..pattern_len];
            
            
            let num_repeats = length / pattern_len;
            let expected_string = pattern.repeat(num_repeats);

            if password == expected_string {
                return true; 
            }
        }
    }
    return false;
}

