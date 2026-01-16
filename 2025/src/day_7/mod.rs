use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

pub mod part1;
pub mod part2;

pub struct Solution;
impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Laboratories"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        7
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        part2::process(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TachyonState {
    Empty,
    Beam,
    Splitter,
    Start,
}

#[derive(Debug, Clone)]
struct TachyonGrid {
    width: usize,
    height: usize,
    slots: Vec<TachyonState>,
}

#[allow(unused)]
impl TachyonGrid {
    fn new(width: usize) -> Self {
        Self {
            width,
            height: 0,
            slots: Vec::new(),
        }
    }

    fn display(&self) {
        for line in self.slots.chunks(self.width) {
            let mut s: String = String::new();
            for state in line.iter() {
                match state {
                    TachyonState::Beam => s.push('|'),
                    TachyonState::Splitter => s.push('^'),
                    TachyonState::Empty => s.push('.'),
                    TachyonState::Start => s.push('S'),
                }
            }
            println!("{}", s);
        }
    }

    fn add_row(&mut self, row: &str) {
        for ch in row.chars() {
            let sl = match ch {
                'S' => TachyonState::Start,
                '|' => TachyonState::Beam,
                '^' => TachyonState::Splitter,
                _ => TachyonState::Empty,
            };
            self.slots.push(sl);
        }
        self.height += 1;
    }

    fn get_slot(&self, x: usize, y: usize) -> Option<TachyonState> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.slots.get(idx).copied()
        }
    }

    fn set_slot(&mut self, x: usize, y: usize, tachyon: TachyonState) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;
        self.slots[idx] = tachyon;
        Some(())
    }
}
