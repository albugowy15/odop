use crate::utils::linked_list::ListNode;

use super::Solution;

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
    let head = crate::utils::linked_list::create_linked_list(vec![5, 4, 2, 1]);

    assert_eq!(Solution::pair_sum(head), 6);

    let head = crate::utils::linked_list::create_linked_list(vec![4, 2, 2, 3]);
    assert_eq!(Solution::pair_sum(head), 7);

    let head = crate::utils::linked_list::create_linked_list(vec![1, 100000]);

    assert_eq!(Solution::pair_sum(head), 100001);
}
