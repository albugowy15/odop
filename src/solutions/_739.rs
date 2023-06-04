use super::Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = Vec::new();

        for (i, t) in temperatures.iter().enumerate() {
            while !stack.is_empty() && *t > stack.last().unwrap().0 {
                let stack_i = stack.pop().unwrap().1;
                answer[stack_i] = (i - stack_i) as i32;
            }
            stack.push((*t, i));
        }

        answer
    }
}

#[test]
fn test_daily_temperature() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let output = vec![1, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(output, Solution::daily_temperatures(temperatures));

    let temperatures = vec![30, 40, 50, 60];
    let output = vec![1, 1, 1, 0];
    assert_eq!(output, Solution::daily_temperatures(temperatures));

    let temperatures = vec![30, 60, 90];
    let output = vec![1, 1, 0];
    assert_eq!(output, Solution::daily_temperatures(temperatures));
}
