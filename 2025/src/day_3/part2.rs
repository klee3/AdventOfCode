pub fn process(input: &str) -> Result<String, String> {
    let mut total: u128 = 0; // 12 digits can exceed u32

    for bank in input.lines() {
        let bytes = bank.as_bytes();
        let n = bytes.len();
        let k = 12;

        if n < k {
            continue;
        }

        let mut chosen: [u8; 12] = [0; 12];
        let mut start = 0;

        for pos in 0..k {
            let end = n - (k - pos);
            let mut best = bytes[start];
            let mut best_idx = start;

            for i in start + 1..=end {
                if bytes[i] > best {
                    best = bytes[i];
                    best_idx = i;
                }
            }

            chosen[pos] = best - b'0';
            start = best_idx + 1;
        }

        // turn 12 digits into a number
        let mut val: u128 = 0;
        for d in chosen {
            val = val * 10 + d as u128;
        }

        total += val;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_3::Solution};

    #[test]
    fn example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "3121910778619");
    }
}
