use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let nums: Vec<Vec<u64>> = get_sorted_list()?;
    let groceries: Vec<u64> = get_groceries()?;
    let merged_list: Vec<Vec<u64>> = merge_lists(&nums);
    let num_fresh: u64 = binary_search(&merged_list, &groceries);

    println!("num_fresh {}", num_fresh);
    
    Ok(())
}

fn binary_search(merged_list: &Vec<Vec<u64>>, groceries: &Vec<u64>) -> u64 {
    let mut num_fresh: u64 = 0;

    for g in groceries.iter() {
        let mut i: usize = 0;
        let mut j: usize = merged_list.len() - 1;

        while i <= j {
            let mid: usize = (i+j)/2;

            let range = merged_list[mid][0]..merged_list[mid][1];

            if range.contains(g) {
                num_fresh += 1;
                break;
            } else if *g < merged_list[mid][0] {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
    }

    num_fresh
}


fn get_sorted_list() -> Result<Vec<Vec<u64>>, std::io::Error> {
    let file_path = "/Users/adityaparekh/Advent/adam/5/cafetaria/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u64>> = Vec::new();

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let line: String = line_result.expect("REASON");
        
        if line.contains("-") {
            let nums =  line.split('-');

            let mut curr: Vec<u64> = Vec::new();

            for num in nums {
                
                let parsed_result: Result<u64, std::num::ParseIntError> = num.parse();
                
                match parsed_result {
                    Ok(res) => {
                        curr.push(res);
                    }
                    Err(_e) => ()
                }
            }

            grid.push(curr);
        } else {
            break;
        }
    }

    grid.sort_by_key(|inner_list| inner_list[0]);

    Ok(grid)
}

fn get_groceries() -> Result<Vec<u64>, std::io::Error> {
    let file_path = "/Users/adityaparekh/Advent/adam/5/cafetaria/src/input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut groceries: Vec<u64> = Vec::new();

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let line: String = line_result.expect("REASON");
        let parsed_result: Result<u64, std::num::ParseIntError> = line.trim().parse();

        match parsed_result {
            Ok(num) => {
                groceries.push(num);
            }
            Err(_e) => ()
        }
    }

    Ok(groceries)
}

fn merge_lists(num_list: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {

    let mut merged_vec: Vec<Vec<u64>> = Vec::new();

    let mut total_fresh: u64 = 0;

    for nums in num_list.iter() {
        let i: u64 = nums[0];
        let j: u64 = nums[1];

        if merged_vec.len() == 0 {
            merged_vec.push(nums.clone());
        } else {
            let x: u64 = merged_vec[merged_vec.len() - 1][0];
            let y: u64 = merged_vec[merged_vec.len() - 1][1];

            if !(x <= y && y < i && i <= j) {
                let min_num: u64 = std::cmp::min(i,x);
                let max_num: u64 = std::cmp::max(j,y);
                merged_vec.pop();
                merged_vec.push(vec![min_num, max_num]);
            } else {
                merged_vec.push(nums.clone());
            }
        }
    }

    for nums in merged_vec.iter() {
        let i: u64 = nums[0];
        let j: u64 = nums[1];

        total_fresh += j-i+1;
    }

    println!("total_fresh {:?}", total_fresh);

    merged_vec
}
