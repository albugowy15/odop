use super::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        let mut prefix = 1;
        for i in 0..n {
            ans[i] = prefix;
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for i in (0..n).rev() {
            ans[i] *= postfix;
            postfix *= nums[i];
        }
        ans
    }
}

#[test]
fn test_product_except_self() {
    let nums = vec![1, 2, 3, 4];
    let expected = vec![24, 12, 8, 6];
    assert_eq!(Solution::product_except_self(nums), expected);

    let nums = vec![-1, 1, 0, -3, 3];
    let expected = vec![0, 0, 9, 0, 0];
    assert_eq!(Solution::product_except_self(nums), expected);
}
