use std::collections::HashMap;

use super::Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let map_bracket = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

        for bracket in s.chars() {
            match map_bracket.get(&bracket) {
                Some(item) => {
                    if !stack.is_empty() && item == stack.last().unwrap() {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                None => stack.push(bracket),
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test_valid_parentheses() {
    let s = String::from("()");
    assert_eq!(true, Solution::is_valid(s));

    let s = String::from("()[]{}");
    assert_eq!(true, Solution::is_valid(s));

    let s = String::from("(]");
    assert_eq!(false, Solution::is_valid(s));

    let s = String::from("){}[]");
    assert_eq!(false, Solution::is_valid(s));
}
