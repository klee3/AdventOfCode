pub fn process(input: &str) -> Result<String, String> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let center = width / 2;

    let mut timelines = vec![0u64; width];
    timelines[center] = 1;

    // Only process every second line and every second tile on each line,
    // starting in the center and growing in a triangle by 1 tile in each direction.
    for (y, row) in lines.iter().skip(2).step_by(2).enumerate() {
        for x in ((center - y)..(center + y + 1)).step_by(2) {
            let count = timelines[x];

            // Not all splitters are reachable, so check that there are beams from above.
            if count > 0 && row[x] == b'^' {
                timelines[x] = 0;
                timelines[x - 1] += count;
                timelines[x + 1] += count;
            }
        }
    }

    let total_timelins: u64 = timelines.iter().sum();

    Ok(total_timelins.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_7::Solution};

    #[test]
    fn example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "40");
    }
}
