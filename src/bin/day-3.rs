use advent_of_code_25::parse_args;

const N_COUNT: usize = 12;

pub fn main() -> anyhow::Result<()> {
    let input = parse_args(3)?
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u128).collect())
        .collect::<Vec<Vec<u128>>>();

    let mut sum = 0;

    for row in input.iter() {
        let len = row.len();

        let mut largest = [0u128; N_COUNT];
        let mut n = 0;
        let mut needle_index = 0;

        while n < N_COUNT {
            let mut largest_local = largest[n];
            let max_index = len - (N_COUNT - n - 1);
            for (i, &value) in row
                .iter()
                .enumerate()
                .skip(needle_index)
                .take_while(|(i, _)| *i < max_index)
            {
                if value > largest_local {
                    largest_local = value;
                    needle_index = i + 1;
                }
            }
            largest[n] = largest_local;
            n += 1;
        }

        let value = largest.map(|v| v.to_string()).join("").parse::<u128>()?;
        sum += value;
    }

    println!("Sum: {}", sum);

    Ok(())
}
