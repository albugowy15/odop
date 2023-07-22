use super::Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                second = num;
            } else {
                return true;
            }
        }

        false
    }
}

#[test]
fn test_increasing_triplet() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::increasing_triplet(nums), true);

    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::increasing_triplet(nums), false);

    let nums = vec![2, 1, 5, 0, 4, 6];
    assert_eq!(Solution::increasing_triplet(nums), true);
}
