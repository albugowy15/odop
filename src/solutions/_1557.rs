use super::Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut is_incoming_edge_exists = vec![false; n as usize];

        for edge in edges {
            is_incoming_edge_exists[edge[1] as usize] = true;
        }

        let mut required_nodes: Vec<i32> = Vec::new();
        for i in 0..n {
            if !is_incoming_edge_exists[i as usize] {
                required_nodes.push(i)
            }
        }

        required_nodes
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
    assert_eq!(
        Solution::find_smallest_set_of_vertices(6, edges),
        vec![0, 3]
    );

    let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
    assert_eq!(
        Solution::find_smallest_set_of_vertices(5, edges),
        vec![0, 2, 3]
    )
}
