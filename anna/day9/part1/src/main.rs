use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day9/input.txt")
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.trim().lines().collect();
    let num_vertices = lines.len();
    let coordinates = get_coordinates(&lines, num_vertices);
    println!("{:?}", coordinates);

    let mut max_area: i128 = 0;
    for a in 0..num_vertices {
        for b in (a+1)..num_vertices {
            let curr_area = calc_area(coordinates[a], coordinates[b]);
            max_area = cmp::max(curr_area, max_area);
        }
    }
    println!("{}", max_area);
}

fn calc_area(vertex_a: (i32, i32), vertex_b: (i32, i32)) -> i128 {
    let dx = (vertex_a.0 - vertex_b.0).abs() + 1;
    let dy = (vertex_a.1 - vertex_b.1).abs() + 1;
    let area: i128 = dx as i128 * dy as i128;
    // println!("a: {:?}, b: {:?}, {}", vertex_a, vertex_b, area);
    return area;
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
