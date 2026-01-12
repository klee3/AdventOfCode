pub fn process(input: &str) -> Result<String, String> {
    let mut columns: Vec<Vec<u128>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let parsed: Result<Vec<u128>, _> = tokens.iter().map(|t| t.parse::<u128>()).collect();
        // Check whether this is a numeric line
        if let Ok(nums) = parsed {
            for (i, value) in nums.into_iter().enumerate() {
                if columns.len() <= i {
                    columns.push(Vec::new());
                }
                columns[i].push(value);
            }
        } else {
            operators = tokens.iter().map(|op| op.chars().next().unwrap()).collect();
        }
    }

    if operators.len() != columns.len() {
        return Err("Operator count does not match column count".into());
    }

    let total: u128 = columns
        .iter()
        .enumerate()
        .map(|(i, col)| -> Result<u128, String> {
            match operators[i] {
                '*' => Ok(col.iter().product()),
                '+' => Ok(col.iter().sum()),
                _ => Err(format!("Unknown operator: {}", operators[i])),
            }
        })
        .collect::<Result<Vec<_>, _>>()?
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
        let result = s.part1(input).unwrap();
        assert_eq!(result, "4277556");
    }
}
