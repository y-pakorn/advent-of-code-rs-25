use std::collections::HashMap;

use advent_of_code_25::parse_args;

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

pub fn main() -> anyhow::Result<()> {
    let is_test = std::env::args().any(|arg| arg == "--test");
    let input = parse_args(8)?;

    let coords: Vec<(i128, i128, i128)> = input
        .lines()
        .map(|line| {
            let [x, y, z] = line
                .split(',')
                .map(|s| s.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
                .try_into()
                .unwrap();
            (x, y, z)
        })
        .collect();

    let n = coords.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0; n];

    // Generate all pairs with their distances
    let mut pairs: Vec<(usize, usize, i128)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1, z1) = coords[i];
            let (x2, y2, z2) = coords[j];
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            pairs.push((i, j, dist_sq));
        }
    }

    // Sort by distance
    pairs.sort_by_key(|(_, _, dist_sq)| *dist_sq);

    // Part 1: Connect the closest pairs
    let num_connections = if is_test { 10 } else { 1000 };
    for (i, j, _) in pairs.iter().take(num_connections) {
        let root_i = find(&mut parent, *i);
        let root_j = find(&mut parent, *j);

        if root_i != root_j {
            // Union by rank
            if rank[root_i] < rank[root_j] {
                parent[root_i] = root_j;
            } else if rank[root_i] > rank[root_j] {
                parent[root_j] = root_i;
            } else {
                parent[root_j] = root_i;
                rank[root_i] += 1;
            }
        }
    }

    // Count circuit sizes for part 1
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        *sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = sizes.values().cloned().collect();
    sizes.sort_unstable();
    sizes.reverse();

    // Multiply the three largest circuits (part 1)
    let result = if sizes.len() >= 3 {
        sizes[0] * sizes[1] * sizes[2]
    } else {
        sizes.iter().product()
    };

    println!("{}", result);

    // Part 2: Continue connecting until all boxes are in one circuit
    let mut last_connection: Option<(usize, usize)> = None;
    for (i, j, _) in pairs.iter().skip(num_connections) {
        let root_i = find(&mut parent, *i);
        let root_j = find(&mut parent, *j);

        if root_i != root_j {
            // Union by rank
            if rank[root_i] < rank[root_j] {
                parent[root_i] = root_j;
            } else if rank[root_i] > rank[root_j] {
                parent[root_j] = root_i;
            } else {
                parent[root_j] = root_i;
                rank[root_i] += 1;
            }

            last_connection = Some((*i, *j));

            // Check if all boxes are now in one circuit
            let mut components = 0;
            for k in 0..n {
                if find(&mut parent, k) == k {
                    components += 1;
                }
            }

            if components == 1 {
                break;
            }
        }
    }

    // Multiply X coordinates of the last connection
    if let Some((i, j)) = last_connection {
        let result2 = coords[i].0 * coords[j].0;
        println!("{}", result2);
    }

    Ok(())
}
