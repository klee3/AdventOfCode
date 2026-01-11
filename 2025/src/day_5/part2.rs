use crate::day_5::IngRange;

pub fn process(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let mut ranges: Vec<IngRange> = Vec::new();

    // Process ranges
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let range = IngRange::from_str(line).ok_or_else(|| format!("invalid range: {}", line))?;
        ranges.push(range);
    }

    ranges.sort_by_key(|r| r.start);
    let mut merged: Vec<IngRange> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.start <= last.end + 1 {
                // overlapping or contiguous
                last.end = last.end.max(r.end);
                continue;
            }
        }
        merged.push(r);
    }

    // Calculate total IDs covered
    let total: u64 = merged.iter().map(|r| r.end - r.start + 1).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_5::Solution};

    #[test]
    fn part1_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "14");
    }
}
