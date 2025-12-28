use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day9/input.txt")
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.trim().lines().collect();
    let num_vertices = lines.len();
    let mut coordinates = get_coordinates(&lines, num_vertices);

    // stores ranges of y allowed for every x-coordinate
    let mut ranges_x: HashMap<i32, (i32, i32)> = HashMap::new();
    // stores ranges of x allowed for every y-coordinate
    let mut ranges_y: HashMap<i32, (i32, i32)> = HashMap::new();
    populate_map(&mut ranges_x, &mut ranges_y, &coordinates);
    iterate_x_coords(&mut ranges_x, &mut ranges_y, &mut coordinates, num_vertices);
    iterate_y_coords(&mut ranges_x, &mut ranges_y, &mut coordinates, num_vertices);

    let mut max_area: i128 = 0;
    for a in 0..num_vertices {
        let (a_x, a_y) = coordinates[a];
        for b in (a + 1)..num_vertices {
            let (b_x, b_y) = coordinates[b];
            // by must be in allowed y range for ax
            let by_allowed = in_range(ranges_x.get(&a_x).unwrap(), b_y);
            let ay_allowed = in_range(ranges_x.get(&b_x).unwrap(), a_y);
            let bx_allowed = in_range(ranges_y.get(&a_y).unwrap(), b_x);
            let ax_allowed = in_range(ranges_y.get(&b_y).unwrap(), a_x);
            // println!(
            //     "a: {:?}, b: {:?}, ax: {}, ay: {}, bx: {}, by: {}",
            //     coordinates[a], coordinates[b], ax_allowed, ay_allowed, bx_allowed, by_allowed
            // );
            if by_allowed && bx_allowed && ax_allowed && ay_allowed {
                let curr_area = calc_area(coordinates[a], coordinates[b]);
                max_area = cmp::max(curr_area, max_area);
            }
        }
    }
    println!("{}", max_area);
}

fn in_range(range: &(i32, i32), value: i32) -> bool {
    return value >= range.0 && value <= range.1;
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
fn populate_map(
    map_x: &mut HashMap<i32, (i32, i32)>,
    map_y: &mut HashMap<i32, (i32, i32)>,
    coordinates: &Vec<(i32, i32)>,
) {
    for coord in coordinates {
        map_x.insert(coord.0, (i32::MAX, i32::MIN));
        map_y.insert(coord.1, (i32::MAX, i32::MIN));
    }
}

fn iterate_x_coords(
    ranges_x: &mut HashMap<i32, (i32, i32)>,
    ranges_y: &mut HashMap<i32, (i32, i32)>,
    coordinates: &mut Vec<(i32, i32)>,
    num_vertices: usize,
) {
    coordinates.sort_by_key(|coord| (coord.0, coord.1));
    let mut i: usize = 0;
    while i < num_vertices {
        let x: i32 = coordinates.get(i).unwrap().0;

        let y_min = coordinates.get(i).unwrap().1;
        let mut y_max = coordinates.get(i).unwrap().1;
        // until y coords are the same
        while (i + 1 < num_vertices)
            && (coordinates.get(i).unwrap().0 == coordinates.get(i + 1).unwrap().0)
        {
            i += 1;
            // update maximum for current x
            y_max = coordinates.get(i).unwrap().1;
        }

        let curr_range = ranges_x.get_mut(&x).unwrap();
        curr_range.0 = cmp::min(curr_range.0, y_min);
        curr_range.1 = cmp::max(curr_range.1, y_max);
        // println!("x: {}, range:{:?}", x, curr_range);

        // update ranges_y for min/max with current x value
        for y in y_min..=y_max {
            if let Some(x_range) = ranges_y.get_mut(&y) {
                x_range.0 = cmp::min(x_range.0, x);
                x_range.1 = cmp::max(x_range.1, x);
                // println!("y: {}, range:{:?}", y, x_range);
            }
        }
        i += 1;
    }
}

fn iterate_y_coords(
    ranges_x: &mut HashMap<i32, (i32, i32)>,
    ranges_y: &mut HashMap<i32, (i32, i32)>,
    coordinates: &mut Vec<(i32, i32)>,
    num_vertices: usize,
) {
    coordinates.sort_by_key(|coord| (coord.1, coord.0));
    let mut i: usize = 0;
    while i < num_vertices {
        let y: i32 = coordinates.get(i).unwrap().1;

        let x_min = coordinates.get(i).unwrap().0;
        let mut x_max = coordinates.get(i).unwrap().0;
        // until y coords are the same
        while (i + 1 < num_vertices)
            && (coordinates.get(i).unwrap().1 == coordinates.get(i + 1).unwrap().1)
        {
            i += 1;
            // update maximum for current x
            x_max = coordinates.get(i).unwrap().0;
        }

        let curr_range = ranges_y.get_mut(&y).unwrap();
        curr_range.0 = cmp::min(curr_range.0, x_min);
        curr_range.1 = cmp::max(curr_range.1, x_max);
        // println!("y: {}, range:{:?}", y, curr_range);

        // update ranges_y for min/max with current x value
        for x in x_min..=x_max {
            if let Some(y_range) = ranges_x.get_mut(&x) {
                y_range.0 = cmp::min(y_range.0, y);
                y_range.1 = cmp::max(y_range.1, y);
                // println!("x: {}, range:{:?}", x, y_range);
            }
        }
        i += 1;
    }
}
