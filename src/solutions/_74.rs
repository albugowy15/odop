use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let matrix = matrix.concat();
        return Self::search_binary(&matrix, target).is_some();
    }
    fn search_binary(nums: &[i32], target: i32) -> Option<usize> {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = (l + r) / 2;
            match nums[m as usize].cmp(&target) {
                Ordering::Greater => r = m,
                Ordering::Less => l = m + 1,
                Ordering::Equal => return Some(m as usize),
            }
        }
        None
    }
}

#[test]
fn test_search_2d_matrix() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    assert_eq!(Solution::search_matrix(matrix, target), true);

    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 13;
    assert_eq!(Solution::search_matrix(matrix, target), false);
}
