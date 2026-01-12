use std::process;

use crate::utils::{advent_of_code_solution::AdventOfCodeSolution, get_solution};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod utils;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Program Usage: {0} <day>", args.first().unwrap());
        process::exit(1);
    }

    // Try to convert argument into integer
    let day: u8 = match args[1].parse() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Program Usage: {0} <day>", args[0]);
            process::exit(1);
        }
    };

    let solution: Box<dyn AdventOfCodeSolution> = match get_solution(day) {
        Some(s) => s,
        None => {
            eprintln!("Day {} not implemented", day);
            process::exit(1);
        }
    };

    let path = format!("inputs/day{}.txt", solution.day());
    let input = std::fs::read_to_string(path).unwrap();

    println!(
        "{0} {1} - {2}",
        solution.year(),
        solution.day(),
        solution.name()
    );
    println!("Part 1: {}", solution.part1(&input)?);
    println!("Part 2: {}", solution.part2(&input)?);

    Ok(())
}
