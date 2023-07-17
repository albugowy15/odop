use super::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut s1_count = [0; 26];
        let mut s2_count = [0; 26];

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 0..s1.len() {
            s1_count[s1_chars[i] as usize - 'a' as usize] += 1;
            s2_count[s2_chars[i] as usize - 'a' as usize] += 1;
        }
        let mut matches = 0;
        for i in 0..26 {
            if s1_count[i] == s2_count[i] {
                matches += 1
            } else {
                matches += 0
            }
        }
        let mut l = 0;
        for r in s1.len()..s2.len() {
            if matches == 26 {
                return true;
            }
            let index = s2_chars[r] as usize - 'a' as usize;
            s2_count[index] += 1;
            if s1_count[index] == s2_count[index] {
                matches += 1;
            } else if s1_count[index] + 1 == s2_count[index] {
                matches -= 1;
            }

            let index = s2_chars[l] as usize - 'a' as usize;
            s2_count[index] -= 1;
            if s1_count[index] == s2_count[index] {
                matches += 1;
            } else if s1_count[index] - 1 == s2_count[index] {
                matches -= 1;
            }
            l += 1;
        }
        matches == 26
    }
}

#[test]
fn test_check_inclusion() {
    assert_eq!(
        Solution::check_inclusion(String::from("ab"), String::from("eidbaooo"),),
        true
    );
    assert_eq!(
        Solution::check_inclusion(String::from("ab"), String::from("eidboaoo"),),
        false
    );
}
