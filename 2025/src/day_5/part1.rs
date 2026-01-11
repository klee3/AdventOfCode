use crate::day_5::IngRange;

pub fn process(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let mut ranges: Vec<IngRange> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    // Process ranges
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let range = IngRange::from_str(line).ok_or_else(|| format!("invalid range: {}", line))?;
        ranges.push(range);
    }

    // Process Ids
    for line in lines {
        let id = line
            .parse::<u64>()
            .map_err(|_| format!("invalid id: {}", line))?;
        ids.push(id);
    }

    ranges.sort_by_key(|r| r.start);
    ids.sort();

    let mut i = 0; // range index
    let mut count = 0;

    for &id in &ids {
        // Move the range pointer forward while ranges end before the current id
        while i < ranges.len() && ranges[i].end < id {
            i += 1;
        }

        // Now either we exhausted ranges or ranges[i].end >= id
        if i < ranges.len() && ranges[i].start <= id && id <= ranges[i].end {
            count += 1;
        }
    }

    Ok(count.to_string())
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
        let result = s.part1(input).unwrap();
        assert_eq!(result, "3");
    }
}
