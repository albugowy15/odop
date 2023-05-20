use std::collections::{HashMap, HashSet, VecDeque};

use super::Solution;

struct Pair {
    first: String,
    second: f64,
}

impl Solution {
    fn build_graph(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
    ) -> HashMap<String, HashMap<String, f64>> {
        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for i in 0..equations.len() {
            let dividend = &equations[i][0];
            let divisor = &equations[i][1];
            let value = values[i];

            if !graph.contains_key(dividend) {
                graph.insert(dividend.to_string(), HashMap::new());
            }
            if !graph.contains_key(divisor) {
                graph.insert(divisor.to_string(), HashMap::new());
            }
            graph
                .get_mut(dividend)
                .unwrap()
                .insert(divisor.to_string(), value);
            graph
                .get_mut(divisor)
                .unwrap()
                .insert(dividend.to_string(), 1.0 / value);
        }
        graph
    }

    fn bfs(start: String, end: String, graph: &HashMap<String, HashMap<String, f64>>) -> f64 {
        let mut q: VecDeque<Pair> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        q.push_back(Pair {
            first: start,
            second: 1.0,
        });

        while !q.is_empty() {
            let node = q.front().unwrap().first.clone();
            let value = q.front().unwrap().second;
            q.pop_front();

            if node == end {
                return value;
            }

            visited.insert(node.clone());

            for neighbor in &graph[&node] {
                let (neighbor_node, neighbor_value) = neighbor;

                if !visited.contains(neighbor_node) {
                    q.push_back(Pair {
                        first: neighbor_node.to_string(),
                        second: value * neighbor_value,
                    })
                }
            }
        }

        -1.0
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let graph = Solution::build_graph(equations, values);
        let mut results: Vec<f64> = Vec::new();

        for query in queries {
            let dividend = &query[0];
            let divisor = &query[1];

            if !graph.contains_key(dividend) || !graph.contains_key(divisor) {
                results.push(-1.0);
            } else {
                results.push(Self::bfs(dividend.to_string(), divisor.to_string(), &graph));
            }
        }
        results
    }
}

#[test]
fn test() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    assert_eq!(
        Solution::calc_equation(equations, values, queries),
        vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
    );

    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
    ];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["c".to_string(), "b".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
        vec!["cd".to_string(), "bc".to_string()],
    ];
    assert_eq!(
        Solution::calc_equation(equations, values, queries),
        vec![3.75000, 0.40000, 5.00000, 0.20000]
    );

    let equations = vec![vec!["a".to_string(), "b".to_string()]];
    let values = vec![0.5];
    let queries = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "c".to_string()],
        vec!["x".to_string(), "y".to_string()],
    ];
    assert_eq!(
        Solution::calc_equation(equations, values, queries),
        vec![0.50000, 2.00000, -1.00000, -1.00000]
    );
}
