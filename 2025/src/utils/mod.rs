use crate::{
    day_1, day_2, day_3, day_4, day_5, day_6, utils::advent_of_code_solution::AdventOfCodeSolution,
};

pub mod advent_of_code_solution;

pub fn get_solution(day: u8) -> Option<Box<dyn AdventOfCodeSolution>> {
    match day {
        1 => Some(Box::new(day_1::Solution)),
        2 => Some(Box::new(day_2::Solution)),
        3 => Some(Box::new(day_3::Solution)),
        4 => Some(Box::new(day_4::Solution)),
        5 => Some(Box::new(day_5::Solution)),
        6 => Some(Box::new(day_6::Solution)),
        _ => None,
    }
}
