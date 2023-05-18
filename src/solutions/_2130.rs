use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = &head;
        let mut v: Vec<i32> = Vec::new();

        while let Some(node) = head {
            v.push(node.val);
            head = &node.next;
        }

        let mut front_pointer = 0;
        let mut back_pointer = v.len() - 1;
        let mut max_sum = 0;

        while front_pointer < back_pointer {
            max_sum = max_sum.max(v[front_pointer] + v[back_pointer]);

            front_pointer += 1;
            back_pointer -= 1;
        }
        max_sum
    }
}

#[test]
fn test() {
    let mut head = Some(Box::new(ListNode::new(5)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(1)));

    assert_eq!(Solution::pair_sum(head), 6);

    let mut head = Some(Box::new(ListNode::new(4)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(3)));

    assert_eq!(Solution::pair_sum(head), 7);

    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(100000)));

    assert_eq!(Solution::pair_sum(head), 100001);
}
