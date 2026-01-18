use kiddo::KdTree;
use kiddo::SquaredEuclidean;

use crate::day_8::UnionFind;
use crate::day_8::process_input;

pub fn process(input: &str) -> Result<String, String> {
    let points = process_input(input)?;

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
    let mut last_edge = None;

    for (_dist, i, j) in edges.iter() {
        if uf.union(*i, *j) {
            // this edge actually connected two separate components
            last_edge = Some((*i, *j));
        }
    }

    // Make sure we found an edge
    let (i, j) = last_edge.ok_or("No connecting edge found")?;

    // Multiply x-coordinates of the two points
    let result = points[i][0] * points[j][0];

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {

    use crate::{AdventOfCodeSolution, day_8::Solution};

    #[test]
    fn example() {
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
        let s = Solution;
        let result = s.part2(input).unwrap();
        assert_eq!(result, "25272");
    }
}
