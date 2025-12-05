use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day2/part1/src/input.txt")
    .expect("Should have been able to read the file");
    let parts: Vec<&str> = contents.trim().split(',').collect();
    let mut result: i64 = 0;
    for range in &parts {
        if let Some((left, right)) = range.split_once('-') {
            let low: i64 = left.parse()?;
            let high: i64 = right.parse()?;
            for i in (low..=high).step_by(1) {
                result += if verify_valid_password(&i.to_string()) { i } else { 0 };
                //println!("{}, {}", low, high, pwd);
            }
            //println!("result: {}", result)
        } else {
            // Handle the error case where the comma is missing
            println!("Invalid range format: {}", range);
        }
    }
    println!("{}", result);
    Ok(())
    }

fn verify_valid_password(password: &str) -> bool {
    let length = password.len();
    if length % 2 != 0 {
        return false;
    }
    let (left, right) = password.split_at(length/2);
    if left == right {
        return true;
    }
    return false;
}