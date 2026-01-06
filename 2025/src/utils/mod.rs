use crate::{day_1, day_2, day_3, utils::advent_of_code_solution::AdventOfCodeSolution};

pub mod advent_of_code_solution;

pub fn get_solution(day: u8) -> Option<Box<dyn AdventOfCodeSolution>> {
    match day {
        1 => Some(Box::new(day_1::Solution)),
        2 => Some(Box::new(day_2::Solution)),
        3 => Some(Box::new(day_3::Solution)),
        _ => None,
    }
}
