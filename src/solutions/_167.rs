use super::Solution;

impl Solution {
    pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first_pt = 0;
        let mut last_pt = numbers.len() - 1;
        while first_pt < last_pt {
            if numbers[first_pt] + numbers[last_pt] > target {
                last_pt -= 1;
            } else if numbers[first_pt] + numbers[last_pt] < target {
                first_pt += 1;
            } else {
                return vec![first_pt as i32 + 1, last_pt as i32 + 1];
            }
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    let numbers = vec![2, 7, 11, 15];
    assert_eq!(vec![1, 2], Solution::two_sum_2(numbers, 9));

    let numbers = vec![2, 3, 4];
    assert_eq!(vec![1, 3], Solution::two_sum_2(numbers, 6));

    let numbers = vec![-1, 0];
    assert_eq!(vec![1, 2], Solution::two_sum_2(numbers, -1));
}
