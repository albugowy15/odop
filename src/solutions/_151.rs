use super::Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[test]
fn test_reverse_words() {
    let s = String::from("the sky is blue");
    assert_eq!(Solution::reverse_words(s), String::from("blue is sky the"));

    let s = String::from("  hello world  ");
    assert_eq!(Solution::reverse_words(s), String::from("world hello"));

    let s = String::from("a good   example");
    assert_eq!(Solution::reverse_words(s), String::from("example good a"));
}
