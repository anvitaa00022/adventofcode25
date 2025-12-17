use std::fs;
use std::error::Error;
use std::io::{self};

fn main() -> io::Result<()> {
    let _ = first_part();
    let _ = second_part();
    
    Ok(())
}

fn second_part() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("/Users/adityaparekh/Advent/adam/6/trash_compactor/src/input.txt")?;

    let mut str_vec: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        str_vec.push(line.chars().collect());
    }

    // figure out the ops first
    let mut op_vec: Vec<u64> = Vec::new();
    let mut sum_vec: Vec<u64> = Vec::new();
    let mut prod_vec: Vec<u64> = Vec::new();

    for chr in str_vec[str_vec.len()-1].iter() {
        if *chr == '+' {
            op_vec.push(1);
        } else if *chr == '*' {
            op_vec.push(2);
        }
    }

    str_vec.pop();
    let mut curr_sum: u64 = 0;
    let mut curr_prod: u64 = 1;
    let mut found_num: bool = false;

    for i in 0..str_vec[0].len() {
        let mut curr_num: u64 = 0;
        found_num = false;

        for j in 0..str_vec.len() {
            let num = str_vec[j][i].to_digit(10);
            match num {
                Some(x) => {
                    curr_num = curr_num*10 + (x as u64);
                    found_num = true;
                }
                None => (),
            }
        }

        if found_num {
            curr_sum += curr_num;
            curr_prod *= curr_num;
        } else {
            sum_vec.push(curr_sum);
            prod_vec.push(curr_prod);
            curr_sum = 0;
            curr_prod = 1;
        }
    }

    if found_num {
        sum_vec.push(curr_sum);
        prod_vec.push(curr_prod);
    }

    let mut ret_sum: u64 = 0;

    for i in 0..op_vec.len() {
        if op_vec[i] == 1 {
            ret_sum += sum_vec[i];
        } else {
            ret_sum += prod_vec[i];
        }
    }

    println!("curr_sum {:?}", ret_sum);

    Ok(())
}

fn first_part() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("/Users/adityaparekh/Advent/adam/6/trash_compactor/src/input.txt")?;

    let mut nums: Vec<Vec<u64>> = Vec::new();
    
    for line in contents.lines() {
        let mut curr: Vec<u64> = Vec::new();

        let mut seen_num: bool = false;
        let mut curr_num: u64 = 0;

        for chr in line.chars() {
            if chr == ' ' {
                if seen_num {
                    curr.push(curr_num);
                    curr_num = 0;
                    seen_num = false;
                }
            } else {
                seen_num = true;

                if chr == '+' {
                    curr_num = 1 as u64;
                } else if chr == '*' {
                    curr_num = 2 as u64;
                } else {
                    let num = chr.to_digit(10);

                    match num {
                        Some(i) => {
                            curr_num = curr_num*10 + (i as u64);
                        }
                        None => (),
                    }
                }
            }
        }

        if seen_num {
            curr.push(curr_num);
        }

        nums.push(curr);
    }

    let mut curr_sum: u64 = 0;
    for i in 0..nums[0].len() {            
        let mut curr: u64 = 1;
        for j in 0..nums.len()-1 {
            if nums[nums.len()-1][i] == 1 {
                curr += nums[j][i];
            } else {
                curr *= nums[j][i];
            }
        }
        if nums[nums.len()-1][i] == 1 {
            curr -= 1;
        }

        curr_sum += curr;
    }

    println!("curr_sum {}", curr_sum);
    Ok(())
}
