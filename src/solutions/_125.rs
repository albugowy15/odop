use super::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let char_s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        char_s.clone().eq(char_s.rev())
    }
}

#[test]
fn test_is_palindrome() {
    let s = String::from("A man, a plan, a canal: Panama");
    assert_eq!(true, Solution::is_palindrome(s));

    let s = String::from("race a car");
    assert_eq!(false, Solution::is_palindrome(s));

    let s = String::from(" ");
    assert_eq!(true, Solution::is_palindrome(s));
}
