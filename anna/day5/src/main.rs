use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string(
        "/Users/anvitaa/git/adventofcode25/anna/day5/input.txt",
    ).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.trim().lines().collect();
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let mut seen_newline: bool = false;
    for line in lines {
        if line == "" {
            seen_newline = true;
            continue;
        }
        if seen_newline {
            ingredients.push(line.parse().unwrap());
        } else {
            let (left, right) = line.split_once('-').expect("invalid");
            fresh_ranges.push((left.parse().unwrap(), right.parse().unwrap()));
        }
    }
    fresh_ranges.sort_by_key(|&(left_val, _)| left_val);
    let mut simplified_fresh_ranges: Vec<(u64, u64)> = Vec::new();
    for &(curr_left, curr_right) in &fresh_ranges {
        if !simplified_fresh_ranges.is_empty() {
            let &(last_left, last_right) = simplified_fresh_ranges.last().unwrap();

            // if curr range is overlapping, consolidate ranges
            if curr_left >= last_left && curr_left <= last_right {
                simplified_fresh_ranges.pop();
                let final_left = cmp::min(last_left, curr_left);
                let final_right = cmp::max(last_right, curr_right);
                simplified_fresh_ranges.push((final_left, final_right));
                continue;
            }
        }
        simplified_fresh_ranges.push((curr_left, curr_right));
    }

    let ranges_len: i32 = simplified_fresh_ranges.len().try_into().unwrap();
    let mut result:u64 = 0;
    for &ingredient in &ingredients {
        if binary_search(0, ranges_len-1, &simplified_fresh_ranges, ingredient) {
            result += 1
        }
    }
    println!("result, part1: {}", result);
    println!("result, part2: {}", num_ingredients(&simplified_fresh_ranges));
}

fn binary_search(lo: i32, hi:i32, ranges: &Vec<(u64, u64)>, num: u64) -> bool {
    if lo > hi {
        return false;
    }
    let mid: i32 = lo + (hi-lo)/2 as i32;
    let (curr_left, curr_right) = ranges[mid as usize];
    if num >= curr_left && num <= curr_right {
        return true;
    } else if num > curr_right {
        return binary_search(mid+1, hi, ranges, num);
    } else {
        return binary_search(lo, mid-1, ranges, num);
    }
}

fn num_ingredients(ranges: &Vec<(u64, u64)>) -> u64{
    let mut result = 0;
    for &(left, right) in ranges {
        result += right - left + 1; 
    }
    return result;
}
