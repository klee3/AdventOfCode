use std::collections::VecDeque;

use crate::day_7::{TachyonGrid, TachyonState};

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
            if grid.get_slot(x, y) == Some(TachyonState::Start) {
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

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_7::Solution};

    #[test]
    fn example() {
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
