use super::Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack = Vec::new();
        let mut res = Vec::new();

        Self::backtrack_parenthesis(&mut stack, &mut res, 0, 0, n);
        return res;
    }
    fn backtrack_parenthesis(
        stack: &mut Vec<String>,
        res: &mut Vec<String>,
        open: i32,
        close: i32,
        n: i32,
    ) {
        if n == open && n == close {
            res.push(stack.join(""));
            return;
        }
        if open < n {
            stack.push(String::from("("));
            Self::backtrack_parenthesis(stack, res, open + 1, close, n);
            stack.pop();
        }
        if close < open {
            stack.push(String::from(")"));
            Self::backtrack_parenthesis(stack, res, open, close + 1, n);
            stack.pop();
        }
    }
}

#[test]
fn test_generate_parenthesis() {
    let result = vec![
        String::from("((()))"),
        String::from("(()())"),
        String::from("(())()"),
        String::from("()(())"),
        String::from("()()()"),
    ];
    assert_eq!(result, Solution::generate_parenthesis(3));

    let result = vec![String::from("()")];
    assert_eq!(result, Solution::generate_parenthesis(1));
}
