use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set: HashSet<char> = HashSet::new();
        let mut left = 0;
        let mut res: i32 = 0;
        let char_s: Vec<char> = s.chars().collect();
        for r in 0..char_s.len() {
            while char_set.contains(&char_s[r]) {
                char_set.remove(&char_s[left]);
                left += 1;
            }
            char_set.insert(char_s[r]);
            res = res.max((r - left + 1) as i32);
        }
        res
    }
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbbb")),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    );
}
