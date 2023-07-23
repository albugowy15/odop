use super::Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // boyer moore voting : https://www.geeksforgeeks.org/boyer-moore-majority-voting-algorithm/
        let mut votes = 0;
        let mut candidate = nums[0];
        let length = nums.len();
        for i in 0..length {
            if votes == 0 {
                candidate = nums[i];
                votes = 1;
            } else if candidate == nums[i] {
                votes += 1;
            } else {
                votes -= 1;
            }
        }
        candidate
    }
}

#[test]
fn test_majority_element() {
    let nums = vec![3, 2, 3];
    assert_eq!(Solution::majority_element(nums), 3);

    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(nums), 2);
}
