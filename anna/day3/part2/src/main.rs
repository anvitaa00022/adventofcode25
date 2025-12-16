use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day3/part1/src/input.txt",
    )
    .expect("Should have been able to read the file");

    let mut result: u64 = 0;

    let banks: Vec<&str> = contents.trim().lines().collect();
    for bank in &banks {
        let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
        let max_volt = get_max_voltage(bank, 0, 12, &mut memo);
        //println!("{}, {}", max_volt, bank);
        result += max_volt;
    }

    println!("{}", result);
}

fn get_max_voltage(
    bank: &str,
    curr_index: usize,
    num_digits: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let key = (curr_index, num_digits);
    if let Some(&v) = memo.get(&(curr_index, num_digits)) {
        return v;
    }
    if (bank.len() - curr_index) < num_digits || num_digits == 0 {
        return 0;
    }
    
    let curr_digit = bank.chars().nth(curr_index).unwrap().to_digit(10).unwrap() as u64;

    let pick = 10u64.pow((num_digits - 1) as u32) * curr_digit + get_max_voltage(bank, curr_index + 1, num_digits - 1, memo);
    let skip = get_max_voltage(bank, curr_index + 1, num_digits, memo);

    let best = pick.max(skip);
    memo.insert(key, best);
    best
}