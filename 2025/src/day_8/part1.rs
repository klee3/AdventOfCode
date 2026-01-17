use kiddo::KdTree;
use kiddo::SquaredEuclidean;

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

pub fn process(input: &str) -> Result<String, String> {
    part1_testable(input, 1000)
}

pub fn part1_testable(input: &str, limit: usize) -> Result<String, String> {
    let points: Vec<[f64; 3]> = input
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
        .collect::<Result<Vec<_>, _>>()?;

    let n = points.len();
    let kdtree: KdTree<_, 3> = (&points).into();

    // Build all edges with distances
    let mut edges = Vec::new();
    for i in 0..n {
        let p = &points[i];
        let nn = kdtree.nearest_n::<SquaredEuclidean>(p, n);
        for neighbor in nn {
            let j = neighbor.item as usize;
            if j <= i {
                continue; // avoid duplicates
            }
            edges.push((neighbor.distance, i, j));
        }
    }

    // Sort edges by distance ascending
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Union-Find to track circuits
    let mut uf = UnionFind::new(n);
    for (_dist, i, j) in edges.iter().take(limit) {
        let _ = uf.union(*i, *j);
    }

    // Compute sizes of all circuits
    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    // Multiply three largest
    let product = sizes.iter().take(3).product::<usize>();
    Ok(product.to_string())
}

#[cfg(test)]
mod tests {
    use super::part1_testable;

    #[test]
    fn part1_example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = part1_testable(input, 10).unwrap();
        assert_eq!(result, "40");
    }
}
