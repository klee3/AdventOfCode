pub fn process(input: &str) -> Result<String, String> {
    let mut sum = 0_u64;
    for line in input.split(',') {
        let (start, end) = line.split_once('-').ok_or("bad range")?;
        let start_num: u64 = start.parse().map_err(|_| "bad number")?;
        let end_num: u64 = end.parse().map_err(|_| "bad number")?;
        for n in start_num..=end_num {
            let num = n.to_string();

            let half_len = num.len() / 2;

            for i in 1..=half_len {
                // can't repeat to fill the whole string
                if num.len() % i != 0 {
                    continue;
                }

                let num_part: Vec<&str> = num
                    .as_bytes()
                    .chunks(i)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap())
                    .collect();

                if all_equal(&num_part) {
                    sum += n;
                    break;
                }
            }
        }
    }
    Ok(sum.to_string())
}

fn all_equal<T: PartialEq>(v: &[T]) -> bool {
    match v.first() {
        None => true, // empty vector → “trivially equal”
        Some(first) => v.iter().all(|x| x == first),
    }
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_2::Solution};

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "4174379265");
    }
}
