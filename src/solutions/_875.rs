use super::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        if piles.len() == h as usize {
            return right;
        }
        while left < right {
            let k = (left + right) / 2;
            let mut sum = 0;
            piles
                .iter()
                .for_each(|banana| sum += (*banana as f64 / k as f64).ceil() as i32);
            if sum <= h {
                right = k;
            } else {
                left = k + 1;
            }
        }
        right
    }
}

#[test]
fn test_min_eating_speed() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    assert_eq!(Solution::min_eating_speed(piles, h), 4);

    let piles = vec![30, 11, 23, 4, 20];
    let h = 5;
    assert_eq!(Solution::min_eating_speed(piles, h), 30);

    let piles = vec![30, 11, 23, 4, 20];
    let h = 6;
    assert_eq!(Solution::min_eating_speed(piles, h), 23);

    let piles = vec![805306368, 805306368, 805306368];
    let h = 1000000000;
    assert_eq!(Solution::min_eating_speed(piles, h), 3);
}
