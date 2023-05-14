use std::cmp::max;

struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn backtrack(nums: &Vec<i32>, mask: i32, pairs_picked: i32, memo: &mut Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if 2 * pairs_picked == n {
            return 0;
        }

        if memo[mask as usize] != -1 {
            return memo[mask as usize];
        }

        let mut max_score = 0;

        for first_index in (0..n).step_by(1) {
            for second_index in (first_index + 1..n).step_by(1) {
                if ((mask >> first_index) & 1) == 1 || ((mask >> second_index) & 1) == 1 {
                    continue;
                }

                let new_mask = mask | (1 << first_index) | (1 << second_index);
                let curr_score = (pairs_picked + 1)
                    * Self::gcd(nums[first_index as usize], nums[second_index as usize]);
                let remaining_score = Self::backtrack(nums, new_mask, pairs_picked + 1, memo);

                max_score = max(max_score, curr_score + remaining_score);
            }
        }

        memo[mask as usize] = max_score;
        max_score
    }

    pub fn max_score(nums: Vec<i32>) -> i32 {
        let memo_size = 1 << nums.len();
        let mut memo = vec![-1; memo_size];
        Self::backtrack(&nums, 0, 0, &mut memo)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_score(vec![1, 2]), 1);
    assert_eq!(Solution::max_score(vec![3, 4, 6, 8]), 11);
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6]), 14);
}
