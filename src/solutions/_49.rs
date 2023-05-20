use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return Vec::new();
        }
        let mut ans: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = vec![0; 26];
            for c in s.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }

            let mut sb = String::new();
            for i in 0..26 {
                sb.push('#');
                sb.push_str(&count[i].to_string());
            }
            let key = sb.to_string();
            if !ans.contains_key(&key) {
                ans.insert(key.clone(), Vec::new());
            }
            ans.get_mut(&key).unwrap().push(s);
        }
        ans.values().cloned().collect()
    }
}

#[test]
fn test_group_anagrams() {
    // Test case 1
    let strs1 = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let expected1 = vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]];
    let result1 = Solution::group_anagrams(strs1);
    assert_eq!(expected1, result1);
}
