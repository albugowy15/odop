use crate::utils::linked_list::ListNode;

use super::Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = None;
        while let Some(next) = head {
            node = Some(Box::new(ListNode {
                next: node,
                val: next.val,
            }));
            head = next.next;
        }
        node
    }
}

#[test]
fn test_reverse_list() {
    use crate::utils::linked_list::{create_linked_list, linked_list_to_vec};

    let list = create_linked_list(vec![1, 2, 3, 4, 5]);
    let head = Solution::reverse_list(list);
    let result = linked_list_to_vec(head);

    assert_eq!(result, vec![5, 4, 3, 2, 1]);

    let list = create_linked_list(vec![1, 2]);
    let head = Solution::reverse_list(list);
    let result = linked_list_to_vec(head);

    assert_eq!(result, vec![2, 1]);

    let list = create_linked_list(vec![]);
    let head = Solution::reverse_list(list);
    let result = linked_list_to_vec(head);

    assert_eq!(result, vec![]);
}
