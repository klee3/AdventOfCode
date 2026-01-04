use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

pub mod part1;
pub mod part2;

pub struct Solution;
impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Gift Shop"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        2
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        // part2::process(input)
        todo!()
    }
}
