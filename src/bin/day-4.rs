use advent_of_code_25::parse_args;

pub fn main() -> anyhow::Result<()> {
    let input = parse_args(4)?;

    let mut grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let width = grid.first().map_or(0, |row| row.len());
    let height = grid.len();

    let mut count = 0;

    loop {
        let mut changed = false;
        let mut next_grid = grid.clone();

        for (j, row) in grid.iter().enumerate() {
            for (i, &cell) in row.iter().enumerate() {
                // Skip empty cells
                if cell == '.' {
                    continue;
                }

                // Check surrounding cells using directional deltas
                let surrounding = (-1..=1)
                    .flat_map(|dj| (-1..=1).map(move |di| (di, dj)))
                    // Skip the cell itself
                    .filter(|&(di, dj)| !(di == 0 && dj == 0))
                    // Filter out cells that are out of bounds
                    .filter_map(|(di, dj)| {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && ni < width as isize && nj >= 0 && nj < height as isize {
                            Some((ni as usize, nj as usize))
                        } else {
                            None
                        }
                    })
                    // Filter out cells that are not paper rolls
                    .filter(|&(ni, nj)| grid[nj][ni] == '@')
                    .count();

                if surrounding < 4 {
                    changed = true;
                    count += 1;
                    next_grid[j][i] = '.';
                }
            }
        }

        if !changed {
            break;
        }
        grid = next_grid;
    }

    println!("Count: {}", count);

    Ok(())
}
