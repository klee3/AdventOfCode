pub fn process(input: &str) -> Result<String, String> {
    let mut sum = 0_u64;
    for line in input.split(',') {
        let (start, end) = line.split_once('-').ok_or("bad range")?;
        let start_num: u64 = start.parse().map_err(|_| "bad number")?;
        let end_num: u64 = end.parse().map_err(|_| "bad number")?;
        for n in start_num..=end_num {
            let num = n.to_string();
            if num.len() % 2 != 0 {
                continue;
            }

            let (s, e) = num.split_at(num.len() / 2);
            if s == e {
                sum += n as u64;
            }
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_2::Solution};

    #[test]
    fn part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let s = Solution;
        let result = s.part1(input).unwrap();
        assert_eq!(result, "1227775554");
    }
}
