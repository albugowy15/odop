struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let sorted_salary = {
            let mut sorted_salary = salary.clone();
            sorted_salary.sort();
            sorted_salary
        };

        let avg = sorted_salary[1..salary.len() - 1].iter().sum::<i32>() as f64
            / (salary.len() - 2) as f64;
        return avg;
    }
}

#[test]
fn test() {
    let salary = vec![4000, 3000, 1000, 2000];
    let result = Solution::average(salary);
    assert_eq!(result, 2500.0);
}
