use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day6/part1/input.txt",
    ).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut operators: Vec<(usize, char)> = get_operators(&lines.get(lines.len()-1).unwrap());

    let rows = lines.len()-1;
    let mut start_col: i32 = (lines[0].len()-1).try_into().unwrap();
    
    let mut final_sum = 0;
    while let Some((end_col, operator)) = operators.pop() {
        println!("start {}, end {}, op {}", start_col, end_col, operator);
        let mut curr_result: u128 = if operator == '+' {0} else {1};
        while start_col >= end_col.try_into().unwrap() {
            let mut num_string: String = "".to_string();
            for row in 0..rows {
                let curr_char = lines[row].chars().nth(start_col as usize).unwrap();
                println!("got char{}, coord {}{}", curr_char, row, start_col);
        
                if curr_char != ' ' {
                    num_string.push(curr_char);
                }
            }
            println!("num_string{}", num_string);
            let num: u128 = if num_string.trim() == "" {0} else {num_string.parse().unwrap()};
            println!("num{}", num);
            if operator == '+' {
                curr_result += num;
            } else if operator == '*' {
                curr_result *= num;
            }
            start_col -= 1;
        }
        final_sum += curr_result;
        start_col -= 1;
    }
    println!("{}", final_sum);
}

fn get_operators(line: &str) -> Vec<(usize, char)> {
    return line.chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect();
}
