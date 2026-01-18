use crate::day_1::Direction;

pub fn process(input: &str) -> Result<String, String> {
    let directions: Vec<Direction> = Direction::get_directions(input)?;
    let mut dial = 50;
    let mut counter = 0_u32;

    for direction in directions {
        match direction {
            Direction::Left(num) => {
                dial = (dial - num).rem_euclid(100);
            }
            Direction::Right(num) => dial = (dial + num).rem_euclid(100),
        }
        if dial == 0 {
            counter += 1;
        }
    }
    Ok(counter.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_1::Solution};

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let s = Solution;
        let result = s.part1(input).unwrap();
        assert_eq!(result, "3");
    }
}
