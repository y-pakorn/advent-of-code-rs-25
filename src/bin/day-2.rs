const INPUT: &str = include_str!("../../inputs/2.txt");
const INPUT_TEST: &str = include_str!("../../inputs/2-t.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT
        .split(',')
        .map(|pair| {
            let (a, b) = pair
                .split_once('-')
                .ok_or_else(|| anyhow::anyhow!("Invalid pair"))?;
            anyhow::Ok((a.parse::<u128>()?, b.parse::<u128>()?))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut cnt = 0;

    for (start, end) in input {
        'outer: for value in start..=end {
            let s = value.to_string();
            let bytes = s.as_bytes();
            let len = bytes.len();

            // Check all possible pattern lengths from 1 to len/2
            for pattern_len in 1..=len / 2 {
                // Only check if len is divisible by pattern_len
                if len % pattern_len != 0 {
                    continue;
                }

                let pattern = &bytes[..pattern_len];
                // Check if all chunks match the pattern
                if bytes
                    .chunks_exact(pattern_len)
                    .all(|chunk| chunk == pattern)
                {
                    cnt += value;
                    continue 'outer;
                }
            }
        }
    }

    println!("Count: {}", cnt);

    Ok(())
}
