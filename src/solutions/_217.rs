struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        // let mut sorted_nums = nums.clone();
        // sorted_nums.sort();
        nums.sort();

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}
