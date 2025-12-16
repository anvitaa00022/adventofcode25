use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day6/input.txt",
    ).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.trim().lines().collect();
    let operators: Vec<&str> = get_operators(&lines.get(lines.len()-1).unwrap());

    let rows = lines.len()-1;
    let cols = operators.len();
    let mut nums: Vec<Vec<u128>> = Vec::new();
    for row in 0..rows {
        nums.push(get_num_row(&lines.get(row).unwrap(), cols));
    }

    let mut final_sum: u128 = 0;
    for col in 0..cols {
        let &curr_opertator = operators.get(col).unwrap();
        let mut curr_result: u128 = if curr_opertator == "+" {0} else {1};
        for row in 0..rows {
            if curr_opertator == "+" {
                curr_result += nums[row][col];
            } else if curr_opertator == "*" {
                curr_result *= nums[row][col];
            }
        }
        final_sum += curr_result;
    }
    // println!("{:?}", operators);
    // println!("{:?}", nums);
    println!("{}", final_sum);
}

fn get_operators(line: &str) -> Vec<&str> {
    let mut operators: Vec<&str> = Vec::new();
    for part in line.split(" ") {
        if part == "" {
            continue;
        }
        operators.push(&part);
    }
    return operators;
}

fn get_num_row(line: &str, cols: usize) -> Vec<u128> {
    let mut num_row: Vec<u128>= vec![0; cols];
    let mut ind: usize = 0;
    for part in line.split(" ") {
        if part == "" {
            continue;
        }
        num_row[ind] = part.parse().unwrap();
        ind += 1;
    }
    return num_row;
}
