use super::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if the length of s and t are not equal, return false
        if s.len() != t.len() {
            return false;
        }

        // split every char in s and t into a vector
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        // sort the vectors
        s_chars.sort();
        t_chars.sort();

        // compare the vectors
        for i in 0..s.len() {
            if s_chars[i] != t_chars[i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
