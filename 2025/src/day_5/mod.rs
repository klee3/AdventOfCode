use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

pub mod part1;
pub mod part2;

pub struct Solution;
impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Cafeteria"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        5
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        part2::process(input)
    }
}

// ingredient Range
#[derive(Debug)]
struct IngRange {
    start: u64,
    end: u64,
}

impl IngRange {
    fn from_str(range: &str) -> Option<Self> {
        let mut parts = range.split('-');

        let start = parts.next()?.parse().ok()?;
        let end = parts.next()?.parse().ok()?;

        Some(Self { start, end })
    }
}
