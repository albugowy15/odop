use crate::utils::linked_list::{create_linked_list, ListNode};

use super::Solution;

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
    let node1 = create_linked_list(vec![1, 2, 3, 4]);
    let node2 = create_linked_list(vec![2, 1, 4, 3]);
    assert_eq!(Solution::swap_pairs(node1), node2);
}
