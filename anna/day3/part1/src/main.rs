use std::fs;

fn main() {
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day3/part1/src/input.txt")
    .expect("Should have been able to read the file");

    let mut result = 0;

    let banks: Vec<&str> = contents.trim().split('\n').collect();
    for bank in &banks {
        result += get_max_voltage(bank);
    }
    println!("{}", result);
}


fn get_max_voltage(bank: &str) -> i32 {
    let mut max_voltage: i32 = 0;
    let mut max_digit: char = '0';

    for curr_digit in bank.chars() {
        let curr_num = curr_digit as u32;
        let max_digit_num = max_digit as u32;

        if max_digit_num > 0 {
            let curr_max_voltage: i32 = format!("{}{}", max_digit, curr_digit).parse().unwrap();
            if curr_max_voltage > max_voltage {
                max_voltage = curr_max_voltage;
            }
        }
        if curr_num > max_digit_num {
            max_digit = curr_digit;
        }
    }
    return max_voltage;
}
