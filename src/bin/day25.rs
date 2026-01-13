use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::f64;
use std::ops::Add;

use advent_of_code_2023::*;
use itertools::Itertools;
use nalgebra::DMatrix;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day25.txt");

/// Unfortunately a slow one. This was an interesting puzzle. The answer is so
/// tantalizingly obvious if you view the graph in, for example, GraphViz, yet
/// the solution is not at all easy.
///
/// I've used closeness centrality to estimate the six most central nodes. I
/// use the Floyd-Warshall algorithm for this, which was not the quickest
/// option. This program produces a correct result based on some shaky
/// assumptions.
///
/// Using BTreeMap instead of HashMap for one data structure because I'm
/// hitting some unpredictable bug that probably has to do with the order of
/// the vertices.
///
/// What a great year for Advent of Code! :-)
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap());
    println!("{:?}", Puzzle::time(PUZZLE));
}

fn floyd_warshall<T: Add<Output = T> + PartialOrd + Clone + Copy>(m: &DMatrix<T>) -> DMatrix<T> {
    let n = m.nrows();
    let mut d = m.clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let distance_through_k = d[(i, k)] + d[(k, j)];
                if d[(i, j)] > distance_through_k {
                    d[(i, j)] = distance_through_k;
                }
            }
        }
    }
    d
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    graph: BTreeMap<String, BTreeSet<String>>,
    m: DMatrix<f64>,
}

impl Puzzle {
    #[allow(dead_code)]
    fn graphviz(&self) {
        println!("graph {{");
        for (u, adj) in self.graph.iter() {
            for v in adj.iter() {
                println!("\t{u} -- {v};");
            }
        }
        println!("}}");
    }

    #[allow(dead_code)]
    fn petgraph_closeness(&self) {
        let g = UnGraphMap::<_, ()>::from_edges(
            self.graph
                .iter()
                .flat_map(|(u, adj)| adj.iter().map(|v| (u.as_str(), v.as_str()))),
        );

        let closeness: BTreeMap<_, _> = self
            .graph
            .keys()
            .map(|u| {
                let delta = dijkstra(&g, u, None, |_| 1);
                let n = g.node_count() as f64;
                let distances = delta.values().cloned().sum::<i32>() as f64;
                let closeness = (n - 1.0) / distances;
                // println!("Closeness score for vertex {} is {}.", u, closeness);
                (u, closeness)
            })
            .collect();

        closeness
            .iter()
            .sorted_by(|&(_, c1), &(_, c2)| c1.total_cmp(c2).reverse())
            .take(6)
            .for_each(|(u, closeness)| {
                println!(
                    "Closeness score (via Petgraph/Dijkstra) for vertex {} is {}.",
                    u, closeness
                )
            });
    }

    fn floyd_warshall_closeness(&mut self) -> usize {
        let n = self.graph.len();
        assert_eq!(n, self.m.nrows());
        assert_eq!(n, self.m.ncols());

        let d: DMatrix<f64> = floyd_warshall(&self.m);

        let vertices = self.graph.keys().cloned().collect_vec();
        let top_6: Vec<_> = d
            .row_iter()
            .enumerate()
            .map(|(i, row)| {
                let closeness = ((n - 1) as f64) / row.sum();
                (i, closeness)
            })
            .sorted_by(|(_, c1), (_, c2)| c1.total_cmp(c2).reverse())
            // .map(|(i, closeness)| {
            //     println!(
            //         "Closeness score (via Floyd-Warshall) for vertex {} was {}.",
            //         vertices[i], closeness
            //     );
            //     i
            // })
            .collect();

        // This little bit of added complexity is to pass the test. The test
        // contains a bunch of ties in the closeness scores, so we will just
        // go ahead and remove all those ties.
        let mut include_ties = 6;
        while top_6[include_ties].1 == top_6[include_ties + 1].1 {
            include_ties += 1;
        }
        let top_6 = &top_6[..include_ties];

        // I think we can overshoot. These are highly connected nodes. If we
        // disconnect each of the top 6 from each of the other top 6, then
        // they might still stay connected to their respective partitions.
        //
        // An earlier version of this program ran Floyd-Warshall a second time
        // to find the components. This is unnecessarily slow. We can walk over
        // the original data source with BFS much faster.

        for (i, j) in top_6.iter().cartesian_product(top_6.iter()) {
            let u = &vertices[i.0];
            let v = &vertices[j.0];
            self.graph.get_mut(u).unwrap().remove(v);
            self.graph.get_mut(v).unwrap().remove(u);
        }
        let mut explored = BTreeSet::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(self.graph.keys().into_iter().next().unwrap());
        while let Some(current) = frontier.pop() {
            explored.insert(current);
            for v in self.graph[current].iter() {
                if !explored.contains(&v) {
                    frontier.push(&v);
                }
            }
        }
        explored.len() * (n - explored.len())

        // for (i, j) in top_6.iter().cartesian_product(top_6.iter()) {
        //     if i != j {
        //         // println!("set {},{} to infinity", vertices[i.0], vertices[j.0]);
        //         self.m[(i.0, j.0)] = f64::INFINITY;
        //     }
        // }

        // let d2 = floyd_warshall(&self.m);
        // let infinites = d2
        //     .row(0)
        //     .fold(0, |acc, i| if i.is_finite() { acc + 1 } else { acc });
        // infinites * (n - infinites)
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for line in input.lines() {
            let (u, adj) = line.split_once(':').unwrap();
            for v in adj.trim().split_ascii_whitespace() {
                graph.entry(u.to_owned()).or_default().insert(v.to_owned());
                graph.entry(v.to_owned()).or_default().insert(u.to_owned());
            }
        }

        let vertices = graph.keys().collect_vec();
        let m = DMatrix::from_fn(graph.len(), graph.len(), |i, j| {
            let u = &vertices[i];
            let v = &vertices[j];
            if u == v {
                0.0
            } else if graph[*u].contains(*v) {
                1.0
            } else {
                f64::INFINITY
            }
        });

        Self {
            part1: None,
            graph,
            m,
        }
    }

    fn solve(mut self) -> Self {
        self.part1 = Some(self.floyd_warshall_closeness());
        self
    }
}

#[cfg(test)]
mod snowverload {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day25.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(54));
    }
}
