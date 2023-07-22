#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_linked_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for i in (0..v.len()).rev() {
        let mut node = ListNode::new(v[i]);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn linked_list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }
    v
}
