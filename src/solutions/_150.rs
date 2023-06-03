use super::Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operands_stack: Vec<i32> = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let second = operands_stack.pop().unwrap();
                    let first = operands_stack.pop().unwrap();
                    operands_stack.push(first + second);
                }
                "-" => {
                    let second = operands_stack.pop().unwrap();
                    let first = operands_stack.pop().unwrap();
                    operands_stack.push(first - second);
                }
                "/" => {
                    let second = operands_stack.pop().unwrap();
                    let first = operands_stack.pop().unwrap();
                    operands_stack.push(first / second);
                }
                "*" => {
                    let second = operands_stack.pop().unwrap();
                    let first = operands_stack.pop().unwrap();
                    operands_stack.push(first * second);
                }
                _ => operands_stack.push(token.parse().unwrap()),
            }
        }
        operands_stack[0]
    }
}

#[test]
fn test_eval_rpn() {
    let tokens = vec![
        String::from("2"),
        String::from("1"),
        String::from("+"),
        String::from("3"),
        String::from("*"),
    ];
    assert_eq!(9, Solution::eval_rpn(tokens));

    let tokens = vec![
        String::from("4"),
        String::from("13"),
        String::from("5"),
        String::from("/"),
        String::from("+"),
    ];
    assert_eq!(6, Solution::eval_rpn(tokens));

    let tokens = vec![
        String::from("10"),
        String::from("6"),
        String::from("9"),
        String::from("3"),
        String::from("+"),
        String::from("-11"),
        String::from("*"),
        String::from("/"),
        String::from("*"),
        String::from("17"),
        String::from("+"),
        String::from("5"),
        String::from("+"),
    ];
    assert_eq!(22, Solution::eval_rpn(tokens));
}
