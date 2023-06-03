struct MinStack {
    main_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            main_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }
    fn push(&mut self, val: i32) {
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
        } else {
            let min = val.min(*self.min_stack.last().unwrap());
            self.min_stack.push(min);
        }
        self.main_stack.push(val);
    }
    fn pop(&mut self) {
        self.main_stack.pop();
        self.min_stack.pop();
    }
    fn top(&self) -> i32 {
        *self.main_stack.last().unwrap()
    }
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[test]
fn test_min_stack() {
    let instructions = vec![
        "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
    ];
    let val = vec![
        vec![],
        vec![-2],
        vec![0],
        vec![-3],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let mut min_stack: MinStack = MinStack::new();
    let mut result = Vec::new();
    result.push(None);

    for (i, step) in instructions.iter().enumerate() {
        match *step {
            "MinStack" => {
                min_stack = MinStack::new();
                result.push(None)
            }
            "push" => {
                min_stack.push(val[i][0]);
                result.push(None);
            }
            "pop" => {
                min_stack.pop();
                result.push(None)
            }
            "top" => {
                result.push(Some(min_stack.top()));
            }
            "getMin" => result.push(Some(min_stack.get_min())),
            __ => (),
        }
    }

    let expected_result = vec![
        None,
        None,
        None,
        None,
        None,
        Some(-3),
        None,
        Some(0),
        Some(-2),
    ];
    assert_eq!(result, expected_result);
}
