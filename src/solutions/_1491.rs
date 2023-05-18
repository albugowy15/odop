use super::Solution;

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        // sort the vector
        salary.sort();

        // Calculate the average salary
        // 1. Create a new vector from index 1 untul salary.len() - 2 (exclude the first and last element)
        // 2. Create an iterator from the new vector
        // 3. Sum all the elements in the iterator
        // 4. Divide the sum by the length of the new vector
        return salary[1..salary.len() - 1].iter().sum::<i32>() as f64 / (salary.len() - 2) as f64;
    }
}

#[test]
fn test() {
    let salary = vec![4000, 3000, 1000, 2000];
    let result = Solution::average(salary);
    assert_eq!(result, 2500.0);
}
