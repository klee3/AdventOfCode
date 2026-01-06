pub fn process(input: &str) -> Result<String, String> {
    let mut total = 0_u32;

    for bank in input.lines() {
        let bytes = bank.as_bytes();
        let mut first = bytes[0];
        let mut first_idx = 0;

        // Get max first digit
        for i in 1..=(bytes.len() - 2) {
            if first < bytes[i] {
                first = bytes[i];
                first_idx = i;

                if first == b'9' {
                    break;
                }
            }
        }

        let mut second = bytes[first_idx + 1];
        // Get second from remaining digit
        for i in (first_idx + 2)..(bytes.len()) {
            if second < bytes[i] {
                second = bytes[i];
            }

            if second == b'9' {
                break;
            }
        }

        let d1 = (first - b'0') as u32;
        let d2 = (second - b'0') as u32;

        let num = d1 * 10 + d2;
        total += num;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_3::Solution};

    #[test]
    fn part1_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let s = Solution;
        let result = s.part1(input).unwrap();
        assert_eq!(result, "357");
    }
}
