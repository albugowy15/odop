use super::Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_len = word1.len();
        let word2_len = word2.len();

        let n = word1_len.min(word2_len);
        let mut ans = String::with_capacity(word1_len + word2_len);
        let word1_chars = word1.chars().collect::<Vec<char>>();
        let word2_chars = word2.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < n {
            ans.push(word1_chars[i]);
            ans.push(word2_chars[i]);
            i += 1;
        }

        if n < word1_len {
            while i < word1_len {
                ans.push(word1_chars[i]);
                i += 1;
            }
        } else if n < word2_len {
            while i < word2_len {
                ans.push(word2_chars[i]);
                i += 1;
            }
        }
        ans
    }
}

#[test]
fn test_merge_alternately() {
    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    assert_eq!(
        Solution::merge_alternately(word1, word2),
        String::from("apbqcr")
    );

    let word1 = String::from("ab");
    let word2 = String::from("pqrs");
    assert_eq!(
        Solution::merge_alternately(word1, word2),
        String::from("apbqrs")
    );

    let word1 = String::from("abcd");
    let word2 = String::from("pq");
    assert_eq!(
        Solution::merge_alternately(word1, word2),
        String::from("apbqcd")
    );
}
