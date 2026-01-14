use std::collections::VecDeque;

pub fn process(input: &str) -> Result<String, String> {
    let width = input.lines().next().ok_or("Input line is empty")?.len();
    let mut grid = TachyonGrid::new(width);
    let mut beam_splitted: u32 = 0;

    // Process and store it in grid
    for line in input.lines() {
        grid.add_row(line);
    }

    let mut queue = VecDeque::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get_slot(x, y) == Some(TachyonState::Beam) {
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        // Beam falls downward
        let ny = y + 1;
        if ny < grid.height {
            if grid.get_slot(x, ny) == Some(TachyonState::Empty) {
                grid.set_slot(x, ny, TachyonState::Beam);
                queue.push_back((x, ny));
            }
        }

        // Splitter reaction
        if y + 1 < grid.height {
            if grid.get_slot(x, y + 1) == Some(TachyonState::Splitter) {
                beam_splitted += 1;

                let sy = y + 1;

                if x > 0 && grid.get_slot(x - 1, sy) == Some(TachyonState::Empty) {
                    grid.set_slot(x - 1, sy, TachyonState::Beam);
                    queue.push_back((x - 1, sy));
                }

                if x + 1 < grid.width && grid.get_slot(x + 1, sy) == Some(TachyonState::Empty) {
                    grid.set_slot(x + 1, sy, TachyonState::Beam);
                    queue.push_back((x + 1, sy));
                }
            }
        }
    }

    Ok(beam_splitted.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TachyonState {
    Empty,
    Beam,
    Splitter,
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
                }
            }
            println!("{}", s);
        }
    }

    fn add_row(&mut self, row: &str) {
        for ch in row.chars() {
            let sl = match ch {
                'S' | '|' => TachyonState::Beam,
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

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_7::Solution};

    #[test]
    fn part1_example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let s = Solution;
        let result = s.part1(input).unwrap();
        assert_eq!(result, "21");
    }
}
