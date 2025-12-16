use advent_of_code_25::parse_args;

pub fn main() -> anyhow::Result<()> {
    let input = parse_args(6)?;

    // inputs are in vertical columns
    let columns: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let total: u128 = (0..columns[0].len())
        .map(|j| {
            let ops: Vec<u128> = columns
                .iter()
                .take(columns.len() - 1)
                .map(|row| row[j].parse())
                .collect::<Result<_, _>>()?;

            let result = match columns[columns.len() - 1][j] {
                "*" => ops.iter().product(),
                "+" => ops.iter().sum(),
                op => anyhow::bail!("Invalid operator: {}", op),
            };
            Ok::<u128, anyhow::Error>(result)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    println!("Total: {}", total);

    let columns_char: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // right to left -> up to down
    let total2: u128 = (0..columns_char[0].len())
        .rev()
        .scan(Vec::new(), |intermediate, j| {
            let ops: String = columns_char
                .iter()
                .take(columns_char.len() - 1)
                .filter_map(|row| {
                    let ch = row[j];
                    (ch != ' ').then_some(ch)
                })
                .collect();

            if ops.is_empty() {
                intermediate.clear();
                return Some(0);
            }

            let value = ops.parse::<u128>().ok()?;
            intermediate.push(value);

            let result = match columns_char[columns_char.len() - 1][j] {
                '*' => intermediate.iter().product(),
                '+' => intermediate.iter().sum(),
                _ => return Some(0),
            };
            Some(result)
        })
        .sum();

    println!("Total 2: {}", total2);

    Ok(())
}
