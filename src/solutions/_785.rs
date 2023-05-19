use std::{collections::VecDeque, vec};

use super::Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors: Vec<i32> = vec![-1; n];

        for i in 0..n {
            if colors[i] == -1 {
                let mut q: VecDeque<i32> = VecDeque::new();
                q.push_back(i as i32);
                colors[i] = 0;

                while !q.is_empty() {
                    let current_vertex = q.pop_front().unwrap();

                    for neighbor in &graph[current_vertex as usize] {
                        if colors[*neighbor as usize] == -1 {
                            colors[*neighbor as usize] = 1 - colors[current_vertex as usize];
                            q.push_back(neighbor.clone());
                        } else if colors[*neighbor as usize] == colors[current_vertex as usize] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    assert_eq!(Solution::is_bipartite(graph), false);

    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    assert_eq!(Solution::is_bipartite(graph), true);
}
