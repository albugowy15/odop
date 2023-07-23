use super::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[count] = nums[i];
                count += 1;
            }
        }
        count as i32
    }
}

#[test]
fn test_remove_element() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 3), 2);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(&mut nums, 2), 5);

    let mut nums = vec![2];
    assert_eq!(Solution::remove_element(&mut nums, 3), 1);
    println!("{:?} hello", nums);
}
