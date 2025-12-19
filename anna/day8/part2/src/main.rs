use std::collections::VecDeque;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("/Users/anvitaa/git/adventofcode25/anna/day8/input.txt")
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.trim().lines().collect();
    let num_vertices = lines.len();
    // sorted weighted edges
    let coordinates: Vec<(i128, i128, i128)> = get_coordinates(&lines, num_vertices);
    // println!("{:?}", coordinates);
    let weighted_edges: Vec<(i128, usize, usize)> = get_weighted_edges(&coordinates, num_vertices);
    // println!("{:?}", weighted_edges);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for edge in weighted_edges {
        queue.push_back((edge.1, edge.2));
    }
    let mut parent: Vec<usize> = vec![0; num_vertices];
    let mut rank: Vec<usize> = vec![0; num_vertices];
    for v in 0..num_vertices {
        parent[v] = v;
    }
    // println!("{:?}", parent);
    let mut wires = 1000;
    let mut num_edges_added = 0;
    while wires > 0 {
        let Some(coord) = queue.pop_front() else {
            break;
        };
        let x = coord.0;
        let y = coord.1;
        if find(&mut parent, x) == find(&mut parent, y) {
            continue;
        }
        union(&mut parent, &mut rank, x, y);
        num_edges_added += 1;
        if num_edges_added == num_vertices - 1 {
            println!(
                "final: x.0: {}, y.0: {}, product: {}",
                coordinates[x].0,
                coordinates[y].0,
                coordinates[x].0 * coordinates[y].0
            );
            break;
        }
        wires -= 1;
    }

    let mut counts: Vec<usize> = vec![0; num_vertices];
    for v in 0..num_vertices {
        let root = find(&mut parent, v);
        counts[root] += 1;
    }
    let mut sorted_counts: Vec<(usize, usize)> = Vec::new();
    for v in 0..num_vertices {
        sorted_counts.push((counts[v], v));
    }
    sorted_counts.sort_by_key(|item| item.0);
    // println!("{:?}", parent);
    // println!("{:?}", rank);
    // println!("{:?}", sorted_counts);

    // println!("{:?}", weighted_edges);
    Ok(())
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    return parent[i];
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    // println!("union {:?}", (x, y));
    let root_x = find(parent, x);
    let root_y = find(parent, y);
    if root_x == root_y {
        return;
    }
    if rank[root_x] < rank[root_y] {
        parent[root_x] = root_y;
    } else if rank[root_x] > rank[root_y] {
        parent[root_y] = root_x;
    } else {
        parent[root_x] = root_y;
        rank[root_y] += 1;
    }
}

fn get_coordinates(lines: &Vec<&str>, num_vertices: usize) -> Vec<(i128, i128, i128)> {
    let mut coordinates: Vec<(i128, i128, i128)> = Vec::new();

    // create map for vertex to coordinate
    for i in 0..num_vertices {
        let coord: Vec<i128> = lines[i]
            .split(',')
            .filter_map(|s| s.trim().parse::<i128>().ok())
            .collect();
        coordinates.push((coord[0], coord[1], coord[2]));
    }

    return coordinates;
}

fn get_weighted_edges(
    coordinates: &Vec<(i128, i128, i128)>,
    num_vertices: usize,
) -> Vec<(i128, usize, usize)> {
    let mut weighted_edges: Vec<(i128, usize, usize)> = Vec::new();

    for x in 0..num_vertices {
        for y in (x + 1)..num_vertices {
            let dx: i128 = coordinates[x].0 - coordinates[y].0;
            let dy: i128 = coordinates[x].1 - coordinates[y].1;
            let dz: i128 = coordinates[x].2 - coordinates[y].2;
            let distance = ((dx * dx) + (dy * dy) + (dz * dz)).isqrt();
            weighted_edges.push((distance, x, y));
        }
    }
    weighted_edges.sort_by_key(|item| item.0);
    return weighted_edges;
}
