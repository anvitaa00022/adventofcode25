use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day9/test.txt")
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.trim().lines().collect();
    let num_vertices = lines.len();
    let coordinates = get_coordinates(&lines, num_vertices);

    // compress grid -- 
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    for &(x, y) in &coordinates {
        x_vals.push(x);
        y_vals.push(y);
    }
    x_vals.sort();
    x_vals.dedup();
    y_vals.sort();
    y_vals.dedup();
    let x_map: HashMap<i32, usize> = x_vals.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map: HashMap<i32, usize> = y_vals.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    let mut compressed_coordinates: Vec<(usize, usize)> = Vec::new();
    for &(x, y) in &coordinates {
        let xi = x_map[&x];
        let yi = y_map[&y];
        compressed_coordinates.push((xi, yi));
    }

    // get all red and green tiles
    let all_coords = get_filled_coordinates(&compressed_coordinates);

    let rows = y_vals.len();
    let cols = x_vals.len();
    println!("got all coordinates, rows: {}, col: {}", rows, cols);
    // println!("all coords, {:?}", all_coords);

    let mut tiles: Vec<Vec<u32>> = vec![vec![0; cols]; rows];
    for &(x, y) in &all_coords {
        tiles[y][x] = 1;
    }

    let mut tile_sum: Vec<Vec<i64>> = vec![vec![0; cols+1]; rows+1];
    for y in 0..rows {
        for x in 0..cols {
            tile_sum[y + 1][x + 1] =
                tiles[y][x] as i64 + tile_sum[y][x + 1] + tile_sum[y + 1][x] - tile_sum[y][x];
        }
    }

    // println!("{:?}", tile_sum);

    let mut max_area: i64 = 0;
    for a in 0..num_vertices {
        for b in (a + 1)..num_vertices {
            let (x1, y1) = coordinates[a];
            let (x2, y2) = coordinates[b];

            // normalize rectangle corners
            let (lo_x, hi_x) = (x1.min(x2), x1.max(x2));
            let (lo_y, hi_y) = (y1.min(y2), y1.max(y2));

            let x1_i = x_map[&lo_x];
            let x2_i = x_map[&hi_x];
            let y1_i = y_map[&lo_y];
            let y2_i = y_map[&hi_y];

            // prefix sum query
            let filled_tiles =
            tile_sum[y2_i + 1][x2_i + 1]
              - tile_sum[y1_i][x2_i + 1]
              - tile_sum[y2_i + 1][x1_i]
              + tile_sum[y1_i][x1_i];

            let area =
                (x2_i - x1_i + 1) as i64*
                (y2_i - y1_i + 1) as i64;

            // rectangle is valid iff ALL tiles are green/red
            if filled_tiles == area {
                let actual_area = 
                    (hi_x - lo_x + 1) as i64*
                    (hi_y - lo_y + 1) as i64;
                max_area = cmp::max(max_area, actual_area);
            }
        }
    }
    println!("{}", max_area);
}

fn get_filled_coordinates(coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut boundary: HashSet<(usize, usize)> = HashSet::new();
    let mut vertical_boundary: HashMap<usize, Vec<usize>> = HashMap::new();
    let n = coordinates.len();

    for i in 0..n {
        // coordinates wrap around in question
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[(i + 1) % n];

        if x1 == x2 {
            // vertical edge
            let lo = y1.min(y2);
            let hi = y1.max(y2);
            for y in lo..=hi {
                boundary.insert((x1, y));
                if y != hi {
                    vertical_boundary.entry(y).or_default().push(x1);
                }
            }
        } else {
            // horizontal edge
            let lo = x1.min(x2);
            let hi = x1.max(x2);
            for x in lo..=hi {
                boundary.insert((x, y1));
            }
        }
    }

    let mut filled = boundary.clone();

    for (y, mut xs) in vertical_boundary {
        xs.sort();

        // Fill between pairs of boundary (non-overlapping)
        for pair in xs.chunks(2) {
            if pair.len() == 2 {
                let (x1, x2) = (pair[0], pair[1]);
                for x in x1..x2 {
                    filled.insert((x, y));
                }
            }
        }
    }
    return filled.into_iter().collect();
}

fn get_coordinates(lines: &Vec<&str>, num_vertices: usize) -> Vec<(i32, i32)> {
    let mut coordinates: Vec<(i32, i32)> = Vec::new();

    // create map for vertex to coordinate
    for i in 0..num_vertices {
        let coord: Vec<i32> = lines[i]
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        coordinates.push((coord[0], coord[1]));
    }

    return coordinates;
}