use super::Solution;

impl Solution {
    pub fn search_rotated_sorted_array(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            }
            if nums[left] <= nums[mid] {
                if target > nums[mid] || target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if target < nums[mid] || target > nums[right] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }
        -1
    }
}

#[test]
fn test_search_rotated_sorted_array() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(Solution::search_rotated_sorted_array(nums, target), 4);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    assert_eq!(Solution::search_rotated_sorted_array(nums, target), -1);

    let nums = vec![1];
    let target = 0;
    assert_eq!(Solution::search_rotated_sorted_array(nums, target), -1);
}
