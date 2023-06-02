use super::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut ans = Vec::new();

        for i in 0..n {
            let first_num = nums[i];
            if i > 0 && nums[i - 1] == first_num {
                continue;
            }
            let mut front_pt = i + 1;
            let mut last_pt = n - 1;
            while front_pt < last_pt {
                let three_sum = first_num + nums[front_pt] + nums[last_pt];
                if three_sum == 0 {
                    ans.push(vec![first_num, nums[front_pt], nums[last_pt]]);
                    front_pt += 1;
                    while nums[front_pt] == nums[front_pt - 1] && front_pt < last_pt {
                        front_pt += 1;
                    }
                } else if three_sum > 0 {
                    last_pt -= 1;
                } else if three_sum < 0 {
                    front_pt += 1;
                }
            }
        }

        ans
    }
}

#[test]
fn test_three_sum() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        Solution::three_sum(nums)
    );

    let nums = vec![0, 1, 1];
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(res, Solution::three_sum(nums));

    let nums = vec![0, 0, 0];
    assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(nums));

    let nums = vec![0, 0, 0, 0];
    assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(nums));
}
