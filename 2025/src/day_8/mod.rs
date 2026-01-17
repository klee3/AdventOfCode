use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

mod part1;
mod part2;

pub struct Solution;

impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Playground"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        8
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        todo!()
        // part2::process(input)
    }
}
