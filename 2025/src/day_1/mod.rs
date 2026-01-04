use crate::utils::advent_of_code_solution::AdventOfCodeSolution;
pub struct Solution;

pub mod part1;
pub mod part2;

impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Secret Entrance"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        1
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        part2::process(input)
    }
}

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    // Get Direction from a input file
    fn get_directions(input: &str) -> Result<Vec<Direction>, String> {
        input
            .lines()
            .map(|line| {
                let dir = line.chars().next().ok_or("empty line".to_string())?;
                let num = line[1..]
                    .parse::<i32>()
                    .map_err(|_| "invalid number".to_string())?;
                match dir {
                    'L' => Ok(Direction::Left(num)),
                    'R' => Ok(Direction::Right(num)),
                    _ => Err(format!("invalid direction: {}", dir)),
                }
            })
            .collect()
    }
}
