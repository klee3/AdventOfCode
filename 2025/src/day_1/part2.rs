use crate::day_1::Direction;

pub fn process(input: &str) -> Result<String, String> {
    let directions: Vec<Direction> = Direction::get_directions(input)?;
    let mut dial = 50;
    let mut counter = 0_u32;

    for direction in directions {
        let num = match direction {
            Direction::Left(num) => -num,
            Direction::Right(num) => num,
        };

        let (new_dial, additional_counters) = spin(dial, num);
        dial = new_dial;
        counter += additional_counters as u32;
    }
    Ok(counter.to_string())
}

const DIAL_TOTAL: i32 = 100;

fn spin(dial: i32, rot: i32) -> (i32, i32) {
    let dial_long = dial + rot;
    let mut revolutions = (dial_long / DIAL_TOTAL).abs();
    let new_dial = dial_long.rem_euclid(DIAL_TOTAL);

    if dial != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (new_dial, revolutions)
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_1::Solution};

    #[test]
    fn part2_example() {
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
        let result = s.part2(input).unwrap();
        assert_eq!(result, "6");
    }
}
