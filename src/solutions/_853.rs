use super::Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pair: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        let mut stack: Vec<f32> = Vec::new();
        pair.sort_unstable_by_key(|(x, _)| -x);
        for (p, s) in pair {
            stack.push((target as f32 - p as f32) / s as f32);
            if stack.len() >= 2 && stack[stack.len() - 1] <= stack[stack.len() - 2] {
                stack.pop();
            }
        }
        stack.len() as i32
    }
}

#[test]
fn test_car_fleet() {
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 1, 3];
    let target = 12;
    assert_eq!(3, Solution::car_fleet(target, position, speed));

    let position = vec![3];
    let speed = vec![3];
    let target = 10;
    assert_eq!(1, Solution::car_fleet(target, position, speed));

    let position = vec![0, 2, 4];
    let speed = vec![4, 2, 1];
    let target = 100;
    assert_eq!(1, Solution::car_fleet(target, position, speed));

    let position = vec![6, 8];
    let speed = vec![3, 2];
    let target = 10;
    assert_eq!(2, Solution::car_fleet(target, position, speed));
}
