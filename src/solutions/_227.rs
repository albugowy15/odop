use super::Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut i = 0;
        let mut curr: i32 = 0;
        let mut prev: i32 = 0;
        let mut res: i32 = 0;
        let mut cur_operation = '+';
        let char_s = s.chars().collect::<Vec<char>>();

        while i < s.len() {
            let cur_char = char_s[i];
            if cur_char.is_digit(10) {
                while i < s.len() && char_s[i].is_digit(10) {
                    curr = curr * 10 + char_s[i].to_digit(10).unwrap() as i32;
                    i += 1;
                }
                i -= 1;
                if cur_operation == '+' {
                    res += curr;
                    prev = curr;
                } else if cur_operation == '-' {
                    res -= curr;
                    prev = -curr;
                } else if cur_operation == '*' {
                    res -= prev;
                    res += prev * curr;
                    prev = curr * prev;
                } else if cur_operation == '/' {
                    res -= prev;
                    res += (prev / curr) as i32;
                    prev = (prev / curr) as i32;
                }

                curr = 0;
            } else if cur_char != ' ' {
                cur_operation = cur_char;
            }
            i += 1;
        }

        res
    }
}

#[test]
fn test_calculate_basic_calculator_2() {
    let s = String::from("3+2*2");
    assert_eq!(Solution::calculate(s), 7);

    let s = String::from(" 3/2 ");
    assert_eq!(Solution::calculate(s), 1);

    let s = String::from(" 3+5 / 2 ");
    assert_eq!(Solution::calculate(s), 5);
}
