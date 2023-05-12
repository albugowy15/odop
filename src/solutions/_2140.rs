struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut f = vec![0 as i64; n + 1];

        for i in (0..n).rev().step_by(1) {
            let (points, brainpower) = (questions[i][0] as i64, questions[i][1] as usize);
            let j = (i + brainpower + 1).min(n);

            f[i] = f[i + 1].max(f[j] + points);
        }

        f[0]
    }
}

#[test]
fn test() {
    let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
    let result = Solution::most_points(questions);
    assert_eq!(result, 7);
}
