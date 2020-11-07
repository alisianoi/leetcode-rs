struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let n1 = Solution::count(&l1);
        let n2 = Solution::count(&l2);

        let (node, carry) = if n1 >= n2 {
            Solution::add_lists(l1, l2, n1 - n2)
        } else {
            Solution::add_lists(l2, l1, n2 - n1)
        };

        if carry == 0 {
            return Some(Box::new(node));
        } else {
            let mut n = ListNode::new(1);
            n.next = Some(Box::new(node));
            return Some(Box::new(n));
        }
    }

    /// precondition: l1 is not shorter than l2
    pub fn add_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        wait: i32,
    ) -> (ListNode, i32) {
        if wait > 0 {
            let node0 = l1.unwrap();
            let (node, carry) = Solution::add_lists(node0.next, l2, wait - 1);
            let total = node0.val + carry;
            return if total >= 10 {
                (
                    ListNode {
                        val: total - 10,
                        next: Some(Box::new(node)),
                    },
                    1,
                )
            } else {
                (
                    ListNode {
                        val: total,
                        next: Some(Box::new(node)),
                    },
                    0,
                )
            };
        } else {
            let node1 = l1.unwrap();
            let node2 = l2.unwrap();
            if node1.next.is_some() && node2.next.is_some() {
                let (node, carry) = Solution::add_lists(node1.next, node2.next, 0);
                let total = node1.val + node2.val + carry;
                if total >= 10 {
                    (
                        ListNode {
                            val: total - 10,
                            next: Some(Box::new(node)),
                        },
                        1,
                    )
                } else {
                    (
                        ListNode {
                            val: total,
                            next: Some(Box::new(node)),
                        },
                        0,
                    )
                }
            } else {
                let total = node1.val + node2.val;
                if total >= 10 {
                    (ListNode::new(total - 10), 1)
                } else {
                    (ListNode::new(total), 0)
                }
            }
        }
    }

    pub fn count(xs: &Option<Box<ListNode>>) -> i32 {
        let mut xs = xs;
        let mut count = 0;
        while let Some(x) = xs {
            count += 1;
            xs = &x.next;
        }

        count
    }
}

fn main() {
    println!("Hello, world!");
}
