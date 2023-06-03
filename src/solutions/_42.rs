use super::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut sum = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left = height[left];
        let mut max_right = height[right];
        while left < right {
            if max_left <= max_right {
                left += 1;
                let temp = max_left - height[left];
                if temp > 0 {
                    sum += temp;
                }
                max_left = max_left.max(height[left]);
            } else {
                right -= 1;
                let temp = max_right - height[right];
                if temp > 0 {
                    sum += temp;
                }
                max_right = max_right.max(height[right]);
            }
        }
        sum
    }
}

#[test]
fn test_trap() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(6, Solution::trap(height));

    let height = vec![4, 2, 0, 3, 2, 5];
    assert_eq!(9, Solution::trap(height));
}
