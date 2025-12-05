use advent_of_code_25::parse_args;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u128,
    end: u128,
}

impl Range {
    fn contains(&self, value: u128) -> bool {
        (self.start..=self.end).contains(&value)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn len(self) -> u128 {
        self.end.saturating_sub(self.start).saturating_add(1)
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.start);

    ranges.into_iter().fold(Vec::new(), |mut merged, range| {
        match merged.last_mut() {
            Some(last) if last.overlaps(&range) => {
                *last = last.merge(range);
            }
            _ => merged.push(range),
        }
        merged
    })
}

pub fn main() -> anyhow::Result<()> {
    let input = parse_args(5)?;
    let mut sections = input.split("\n\n");

    let ranges: Vec<Range> = sections
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing ranges section"))?
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .ok_or_else(|| anyhow::anyhow!("Invalid range format: {}", line))?;
            Ok(Range {
                start: start.parse()?,
                end: end.parse()?,
            })
        })
        .collect::<Result<_, anyhow::Error>>()?;

    let ids: Vec<u128> = sections
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing IDs section"))?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let merged_ranges = merge_ranges(ranges);

    let count = ids
        .iter()
        .filter(|&&id| {
            merged_ranges
                .iter()
                .take_while(|range| range.start <= id)
                .any(|range| range.contains(id))
        })
        .count();
    println!("Count: {}", count);

    let total_count: u128 = merged_ranges.iter().map(|r| r.len()).sum();
    println!("Merged range count: {}", total_count);
    Ok(())
}
