use std::fs;
use std::error::Error;
use std::io::{self};
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut adj_list: Vec<Vec<usize>> = Vec::new();
    
    match construct_graph(&mut adj_list) {
        Ok(num) => {
            let res: usize = bfs(&adj_list, &num);
            println!("res {}", res);

            let mut visited: HashMap<(usize, usize), u64> = HashMap::new();
            let res2: u64 = dfs(&adj_list, &num, &mut visited);
            println!("res2 {}", res2);
        }
        Err(_e) => {
            return Ok(());
        }
    };

    Ok(())
}

fn dfs(adj_list: &Vec<Vec<usize>>, start: &(usize, usize), visited: &mut HashMap<(usize, usize), u64>) -> u64 {
    let mut curr: u64 = 0;

    if visited.contains_key(start) {
        return visited[start];
    }

    let m: usize = adj_list.len();
    let n: usize = adj_list[0].len();
    let i: usize = start.0;
    let j: usize = start.1;

    if i+1 == m && j >= 0 && j < n {
        visited.insert(*start, 1 as u64);
        return visited[start];
    }

    if i >= 0 && i < m && j >= 0 && j < n {
        if adj_list[i][j] == 1 {
            curr += dfs(adj_list, &(i,j-1), visited);
            curr += dfs(adj_list, &(i,j+1), visited);
        } else {
            curr += dfs(adj_list, &(i+1,j), visited);
        }
    }    

    visited.insert(*start, curr);
    curr
}

// Returns the start coordinates
fn construct_graph(adj_list: &mut Vec<Vec<usize>>) -> Result<(usize, usize), Box<dyn Error>> {
    let contents = fs::read_to_string("/Users/adityaparekh/Advent/adam/7/laboratories/src/input3.txt")?;

    let mut start: (usize, usize) = (0,0);

    let mut i: usize = 0;
    let mut j: usize = 0;

    for line in contents.lines() {
        let mut curr: Vec<usize> = Vec::new();
        for chr in line.chars() {
            if chr == '.' {
                curr.push(0);
            } else if chr == '^' {
                curr.push(1);
            } else if chr == 'S' {
                start.0 = i;
                start.1 = j;
                curr.push(0);
            }
            j += 1;
        }
        i += 1;
        adj_list.push(curr);
    }

    Ok(start)
}

fn bfs(adj_list: &Vec<Vec<usize>>, start: &(usize, usize)) -> usize {
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    to_visit.push_back(*start);

    let m: usize = adj_list.len();
    let n: usize = adj_list[0].len();
    let mut ctr: usize = 0;

    while to_visit.len() > 0 {
        let curr: (usize, usize) = to_visit.pop_front().expect("REASON");
        let i: usize = curr.0;
        let j: usize = curr.1;

        if adj_list[i][j] == 1 {
            ctr += 1;
            if j-1 >= 0 && !visited.contains(&(i,j-1)) {
                to_visit.push_back((i,j-1));
                visited.insert((i,j-1));
            }
            if j+1 < n && !visited.contains(&(i,j+1)) {
                to_visit.push_back((i,j+1));
                visited.insert((i,j+1));
            }
        } else {
            if i+1 < m && !visited.contains(&(i+1,j)) {
                to_visit.push_back((i+1,j));
                visited.insert((i+1,j));
            }
        }
    }
    ctr
}