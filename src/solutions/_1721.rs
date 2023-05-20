use crate::utils::linked_list::ListNode;

use super::Solution;

impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut curr = &head;

        while let Some(n) = curr {
            len += 1;
            curr = &n.next;
        }

        let (first_index, second_index) = (k - 1, len - k);
        if first_index == second_index {
            return head;
        }
        let (mut v1, mut v2, mut curr) = (0, 0, &head);
        let mut i = 0;

        while let Some(n) = curr {
            if i == first_index {
                v1 = n.val;
            } else if i == second_index {
                v2 = n.val;
            }
            i += 1;
            curr = &n.next;
        }
        if v1 == v2 {
            return head;
        }

        let mut curr = &mut head;
        i = 0;
        while let Some(n) = curr {
            if i == first_index {
                n.val = v2;
            } else if i == second_index {
                n.val = v1;
            }
            i += 1;
            curr = &mut n.next;
        }
        head
    }
}

#[test]
fn test() {
    let head = crate::utils::linked_list::create_linked_list(vec![1, 2, 3, 4, 5]);
    let k = 2;
    let expected = crate::utils::linked_list::create_linked_list(vec![1, 4, 3, 2, 5]);
    assert_eq!(Solution::swap_nodes(head.clone(), k), expected);
}
