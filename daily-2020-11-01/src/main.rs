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

struct Solution {}

// 110 = 2 + 4 = 6
// (1 * 2 + 1) * 2 + 0 = 6
// 111 = 7
// (1 * 2 + 1) * 2 + 1 = 7
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result: i32 = 0;
        let mut head = head;
        while let Some(node) = head {
            result = 2 * result + node.val;
            head = node.next;
        }

        result
    }
}

fn main() {
    let mut node0 = ListNode::new(1);
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(0);
    node1.next = Some(Box::new(node2));
    node0.next = Some(Box::new(node1));
    println!("{:#?}", Solution::get_decimal_value(None));
}
