use crate::day_4::{Grid, SlotState};

pub fn process(input: &str) -> Result<String, String> {
    let first_line = input.lines().next().ok_or("Input is empty")?;
    let line_len = first_line.chars().count();
    let mut grid = Grid::new_with_width(line_len);

    let mut accessible = 0_u32;

    for line in input.lines() {
        if let Err(e) = grid.add_row(line) {
            panic!("{}", e);
        }
    }

    for (y, row) in grid.slots.chunks(grid.width).enumerate() {
        for (x, &slot) in row.iter().enumerate() {
            if slot == SlotState::Empty {
                continue;
            }

            let paper_neighbors = adjacent_paper_count(&grid, x, y);
            // Rule: forklift can access only if fewer than 4 adjacent paper rolls
            if paper_neighbors < 4 {
                accessible += 1;
            }
        }
    }

    Ok(accessible.to_string())
}

fn adjacent_paper_count(grid: &Grid, x: usize, y: usize) -> usize {
    const ADJACENT_OFFSETS: &[(isize, isize)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    ADJACENT_OFFSETS
        .iter()
        .filter(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // skip out-of-bounds
            if nx < 0 || ny < 0 {
                return false;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            grid.get_slot(nx, ny) == Some(SlotState::PaperRoll)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_4::Solution};

    #[test]
    fn example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let s = Solution;
        let result = s.part1(input).unwrap();
        assert_eq!(result, "13");
    }
}
