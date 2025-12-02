const INPUT: &str = include_str!("../../inputs/1.txt");

#[inline]
fn wrap_position(pos: isize) -> isize {
    ((pos % 100) + 100) % 100
}

fn parse_instruction(line: &str) -> anyhow::Result<(char, isize)> {
    let mut chars = line.chars();
    let dir = chars.next().ok_or_else(|| anyhow::anyhow!("Empty line"))?;
    let amount = chars.as_str().parse::<isize>()?;
    Ok((dir, amount))
}

fn count_zeros_during_rotation(start: isize, step: isize, amount: isize) -> usize {
    // During rotation, after each click i (1..=amount), position is wrap(start + i*step)
    // We need to count how many i in 1..=amount satisfy wrap(start + i*step) == 0
    // This means: start + i*step ≡ 0 (mod 100)
    // So: i*step ≡ -start (mod 100)

    let start_wrapped = wrap_position(start);

    // For step = -1 (L): i*(-1) ≡ -start_wrapped (mod 100) => i ≡ start_wrapped (mod 100)
    // For step = 1 (R): i ≡ -start_wrapped (mod 100) => i ≡ (100 - start_wrapped) % 100 (mod 100)
    let target = if step == -1 {
        start_wrapped
    } else {
        (100 - start_wrapped) % 100
    };

    // Count how many i in 1..=amount satisfy i ≡ target (mod 100)
    if target == 0 {
        // Special case: i ≡ 0 (mod 100) means i = 100, 200, ...
        (amount / 100) as usize
    } else if target > amount {
        0
    } else {
        // First occurrence at i = target, then every 100 steps: target, target+100, target+200, ...
        ((amount - target) / 100 + 1) as usize
    }
}

fn main() -> anyhow::Result<()> {
    let mut position = 50isize;
    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in INPUT.lines() {
        let (dir, amount) = parse_instruction(line)?;

        // Save position before rotation for Part 2
        let start_pos = position;

        // Part 1: Check if rotation ends at 0
        match dir {
            'L' => position -= amount,
            'R' => position += amount,
            _ => anyhow::bail!("Invalid direction: {}", dir),
        }
        position = wrap_position(position);
        if position == 0 {
            part1_count += 1;
        }

        // Part 2: Count zeros during rotation
        let step = if dir == 'L' { -1 } else { 1 };
        part2_count += count_zeros_during_rotation(start_pos, step, amount);
    }

    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_count);

    Ok(())
}
