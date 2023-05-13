struct Solution;

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        return salary[1..salary.len() - 1].iter().sum::<i32>() as f64 / (salary.len() - 2) as f64;
    }
}

#[test]
fn test() {
    let salary = vec![4000, 3000, 1000, 2000];
    let result = Solution::average(salary);
    assert_eq!(result, 2500.0);
}
