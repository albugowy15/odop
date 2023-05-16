struct Solution;

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut cloned_head = head.clone();
        let mut cloned_head_pointer = &mut cloned_head;

        while let Some(node) = cloned_head_pointer {
            if node.next.is_none() {
                return cloned_head;
            }
            let mut temp = node.next.as_mut().unwrap().next.clone();
            std::mem::swap(&mut node.next, &mut temp);

            temp.as_mut().unwrap().next = Some(Box::new(*node.clone()));
            std::mem::swap(node, &mut temp.as_mut().unwrap());

            cloned_head_pointer = &mut node.next.as_mut().unwrap().next;
        }

        cloned_head
    }
}

#[test]
fn test() {
    let mut node1 = Some(Box::new(ListNode::new(1)));
    node1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    node1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    node1
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));

    let mut node2 = Some(Box::new(ListNode::new(2)));
    node2.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    node2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    node2
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(3)));

    assert_eq!(Solution::swap_pairs(node1), node2);
}
