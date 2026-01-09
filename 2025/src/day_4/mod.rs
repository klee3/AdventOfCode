use core::fmt;

use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

pub mod part1;
pub mod part2;

pub struct Solution;
impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Printing Department"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        4
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        part2::process(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotState {
    PaperRoll,
    Empty,
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    slots: Vec<SlotState>,
}

#[derive(Debug)]
enum GridError {
    WrongWidth { expected: usize, found: usize },
    InvalidSlotChar { ch: char },
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridError::WrongWidth { expected, found } => {
                write!(
                    f,
                    "row width mismatch: expected {}, got {}",
                    expected, found
                )
            }
            GridError::InvalidSlotChar { ch } => {
                write!(f, "invalid slot character '{}'", ch)
            }
        }
    }
}

#[allow(dead_code)]
impl Grid {
    fn add_row(&mut self, input: &str) -> Result<(), GridError> {
        let found = input.chars().count();
        if found != self.width {
            return Err(GridError::WrongWidth {
                expected: self.width,
                found,
            });
        }

        for ch in input.chars() {
            let slot = match ch {
                '@' => SlotState::PaperRoll,
                '.' => SlotState::Empty,
                _ => {
                    return Err(GridError::InvalidSlotChar { ch });
                }
            };

            self.slots.push(slot);
        }
        self.height += 1;
        Ok(())
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            slots: Vec::new(),
        }
    }

    fn new_with_width(width: usize) -> Self {
        Self {
            height: 0,
            width,
            slots: Vec::new(),
        }
    }

    fn display(&self) {
        for line in self.slots.chunks(self.width) {
            println!("{line:?}");
        }
    }

    fn get_left(&self, x: usize, y: usize) -> Option<SlotState> {
        if x == 0 || y >= self.height {
            return None;
        }

        let idx = y * self.width + (x - 1);
        self.slots.get(idx).copied()
    }

    fn get_right(&self, x: usize, y: usize) -> Option<SlotState> {
        if x + 1 >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + (x + 1);
        self.slots.get(idx).copied()
    }

    fn get_top(&self, x: usize, y: usize) -> Option<SlotState> {
        if y == 0 || x >= self.width {
            return None;
        }

        let idx = (y - 1) * self.width + x;
        self.slots.get(idx).copied()
    }

    fn get_bottom(&self, x: usize, y: usize) -> Option<SlotState> {
        if y + 1 >= self.height || x >= self.width {
            return None;
        }

        let idx = (y + 1) * self.width + x;
        self.slots.get(idx).copied()
    }

    // One to remove all
    fn get_slot(&self, x: usize, y: usize) -> Option<SlotState> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.slots.get(idx).copied()
        }
    }

    fn set_slot(&mut self, x: usize, y: usize, slot: SlotState) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;
        self.slots[idx] = slot;
        Some(())
    }
}
