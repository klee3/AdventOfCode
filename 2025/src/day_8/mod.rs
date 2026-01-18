use crate::utils::advent_of_code_solution::AdventOfCodeSolution;

mod part1;
mod part2;

pub struct Solution;

impl AdventOfCodeSolution for Solution {
    fn name(&self) -> &'static str {
        "Playground"
    }
    fn year(&self) -> u16 {
        2025
    }
    fn day(&self) -> u8 {
        8
    }

    fn part1(&self, input: &str) -> Result<String, String> {
        part1::process(input)
    }

    fn part2(&self, input: &str) -> Result<String, String> {
        part2::process(input)
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return false; // already connected
        }

        // union by size
        if self.size[x_root] < self.size[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parent[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            sizes[root] = self.size[root];
        }
        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

fn process_input(input: &str) -> Result<Vec<[f64; 3]>, String> {
    input
        .lines()
        .map(|line| {
            let v: Vec<f64> = line
                .split(',')
                .map(|n| n.parse::<f64>().map_err(|e| e.to_string()))
                .collect::<Result<_, _>>()?;
            if v.len() != 3 {
                return Err("each line must have exactly 3 numbers".to_string());
            }
            Ok([v[0], v[1], v[2]])
        })
        .collect::<Result<Vec<_>, _>>()
}
