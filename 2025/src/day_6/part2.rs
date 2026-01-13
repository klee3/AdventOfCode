pub fn process(input: &str) -> Result<String, String> {
    // Step 0: get operators from last line
    let last_line = input.lines().last().ok_or("Input is empty")?;
    let operators: Vec<char> = last_line.chars().filter(|c| !c.is_whitespace()).collect();

    // Step 1: Get numeric rows
    let rows: Vec<&str> = input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .all(|token| token.parse::<u32>().is_ok())
        })
        .collect();

    if rows.is_empty() {
        return Err("No numeric rows found".into());
    }

    let row_len = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Step 2: Determine column boundaries based on digit positions
    let mut column_ranges: Vec<(usize, usize)> = Vec::new();
    let mut in_column = false;
    let mut start = 0;

    for i in 0..row_len {
        // check if any row has a digit at this index
        let is_digit_here = rows
            .iter()
            .any(|row| row.chars().nth(i).map(|c| c.is_digit(10)).unwrap_or(false));

        if is_digit_here && !in_column {
            in_column = true;
            start = i;
        } else if !is_digit_here && in_column {
            in_column = false;
            column_ranges.push((start, i)); // end is exclusive
        }
    }

    // handle last column if it runs to the end
    if in_column {
        column_ranges.push((start, row_len));
    }

    // Step 3: Extract column values (preserving alignment)
    let mut columns: Vec<Vec<String>> = Vec::new();

    for &(start, end) in &column_ranges {
        let mut col: Vec<String> = Vec::new();

        for row in &rows {
            // slice safely, if row is too short get empty string
            let s = row.get(start..end).unwrap_or("");
            col.push(s.to_string());
        }

        columns.push(col);
    }

    // Step 4: Check operator-column consistency
    if operators.len() != columns.len() {
        return Err("Operator count does not match column count".into());
    }

    // Step 5: Compute column totals using operators
    let total: u128 = columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            // Determine max width of strings in this column
            let width = col.iter().map(|s| s.len()).max().unwrap_or(0);

            // Reconstruct numbers vertically (digit by digit)
            let nums: Result<Vec<u128>, String> = (0..width)
                .map(|i| {
                    let s: String = col
                        .iter()
                        // safely get char, use '0' if missing
                        .map(|n| n.chars().nth(i).unwrap_or('0'))
                        .collect();
                    // Try parsing; propagate error if invalid
                    s.trim()
                        .parse::<u128>()
                        .map_err(|e| format!("Failed to parse '{}': {}", s, e))
                })
                .collect();

            let nums = nums?;

            // Apply operator
            match operators[i] {
                '*' => Ok(nums.iter().fold(1u128, |acc, &x| acc * x)),
                '+' => Ok(nums.iter().fold(0u128, |acc, &x| acc + x)),
                op => Err(format!("Unknown operator: {}", op)),
            }
        })
        .collect::<Result<Vec<u128>, String>>()?
        .into_iter()
        .sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{AdventOfCodeSolution, day_6::Solution};

    #[test]
    fn part1_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "3263827");
    }
}
