use super::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut min_num = nums[0];

        while left <= right {
            if nums[left] < nums[right] {
                min_num = min_num.min(nums[left]);
                break;
            }
            let mid = (left + right) / 2;
            min_num = min_num.min(nums[mid]);
            if nums[mid] >= nums[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        min_num
    }
}

#[test]
fn test_find_min() {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(Solution::find_min(nums), 1);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(Solution::find_min(nums), 0);

    let nums = vec![11, 13, 15, 17];
    assert_eq!(Solution::find_min(nums), 11);
}
