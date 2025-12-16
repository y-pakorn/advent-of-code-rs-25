use std::collections::HashMap;

use advent_of_code_25::parse_args;

fn traverse_back(
    cache: &mut HashMap<(usize, usize), usize>,
    chars: &[Vec<char>],
    row: usize,
    col: usize,
) -> anyhow::Result<usize> {
    if let Some(&count) = cache.get(&(row, col)) {
        return Ok(count);
    }

    if row == 0 {
        return Ok(0);
    }

    let mut count = 0;

    // Check up first, if current cell is got down from above, can travel up.
    match chars[row - 1][col] {
        '|' => count += traverse_back(cache, chars, row - 1, col)?,
        'S' => return Ok(1),
        _ => {}
    }

    // If not got down from above, check left and right
    if col > 0 && chars[row][col - 1] == '^' {
        count += traverse_back(cache, chars, row - 1, col - 1)?;
    }

    if col < chars[row].len().saturating_sub(1) && chars[row][col + 1] == '^' {
        count += traverse_back(cache, chars, row - 1, col + 1)?;
    }

    cache.insert((row, col), count);
    Ok(count)
}

pub fn main() -> anyhow::Result<()> {
    let input = parse_args(7)?;

    let mut chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = chars.len();
    let cols = chars[0].len();

    let mut count = 0;
    for j in 0..rows.saturating_sub(1) {
        for i in 0..cols {
            match chars[j][i] {
                'S' => {
                    chars[j + 1][i] = '|';
                }
                '|' => match chars[j + 1][i] {
                    '^' => {
                        let mut is_splitted = false;

                        if i > 0 && chars[j + 1][i - 1] != '|' {
                            chars[j + 1][i - 1] = '|';
                            is_splitted = true;
                        }

                        if i < cols - 1 && chars[j + 1][i + 1] != '|' {
                            chars[j + 1][i + 1] = '|';
                            is_splitted = true;
                        }

                        if is_splitted {
                            count += 1;
                        }
                    }
                    _ => chars[j + 1][i] = '|',
                },
                _ => {}
            }
        }
    }

    for row in &chars {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Count: {}", count);

    let mut cache = HashMap::new();
    let variants: usize = (0..cols)
        .filter_map(|i| {
            if chars[rows - 1][i] == '|' {
                traverse_back(&mut cache, &chars, rows - 1, i).ok()
            } else {
                None
            }
        })
        .sum();

    println!("Variants: {}", variants);
    Ok(())
}
