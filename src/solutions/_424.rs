use std::collections::HashMap;

use super::Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut res = 0;
        let mut left = 0;
        let char_s = s.chars().collect::<Vec<char>>();
        let mut max_value = 0;
        for r in 0..char_s.len() {
            count.entry(char_s[r]).and_modify(|c| *c += 1).or_insert(1);
            max_value = max_value.max(*count.get(&char_s[r]).unwrap());

            if r - left + 1 - max_value > k as usize {
                *count.get_mut(&char_s[left]).unwrap() -= 1;
                left += 1;
            }

            res = res.max(r - left + 1);
        }
        res as i32
    }
}

#[test]
fn test_character_replacement() {
    assert_eq!(Solution::character_replacement(String::from("ABAB"), 2), 4);
    assert_eq!(
        Solution::character_replacement(String::from("AABABBA"), 1),
        4
    );
}
