use super::Solution;

impl Solution {
    pub fn count_good_string(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0 as i32; (high + 1) as usize];
        dp[0] = 1;
        let div_mode = 1_000_000_007;

        for end in (1..=high as usize).step_by(1) {
            if end >= zero as usize {
                dp[end] += dp[end - zero as usize];
            }
            if end >= one as usize {
                dp[end] += dp[end - one as usize];
            }
            dp[end] %= div_mode;
        }

        let mut ans = 0;
        for i in (low..=high).step_by(1) {
            ans += dp[i as usize];
            ans %= div_mode;
        }
        ans
    }
}

#[test]
fn test() {
    let (low, high, zero, one) = (2, 3, 1, 2);
    let result = Solution::count_good_string(low, high, zero, one);
    assert_eq!(result, 5);
}
