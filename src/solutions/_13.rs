use super::Solution;

impl Solution {
    fn matcher(roman: char) -> i32 {
        match roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0, // Handle invalid characters gracefully
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut prev = 0;
        s.chars().rev().for_each(|c| {
            let curr_decimal = Self::matcher(c);
            if curr_decimal < prev {
                total -= curr_decimal;
            } else {
                total += curr_decimal;
            }
            prev = curr_decimal;
        });
        total
    }
}

#[test]
fn test_roman_to_int() {
    let s = String::from("MCMXCIV");
    assert_eq!(Solution::roman_to_int(s), 1994);

    let s = String::from("LVIII");
    assert_eq!(Solution::roman_to_int(s), 58);

    let s = String::from("III");
    assert_eq!(Solution::roman_to_int(s), 3);

    let s = String::from("V");
    assert_eq!(Solution::roman_to_int(s), 5);

    let s = String::from("IV");
    assert_eq!(Solution::roman_to_int(s), 4);
}
