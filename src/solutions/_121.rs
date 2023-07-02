use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;
        let mut max_profit = 0;

        while right <= prices.len() - 1 {
            if prices[left] > prices[right] {
                max_profit = max_profit.max(prices[right] - prices[left]);
                left += 1;
                right = left + 1;
            } else {
                right += 1;
            }
        }

        max_profit
    }
}

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
}
