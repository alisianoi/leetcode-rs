// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, tilt) = Solution::compute_tilt(&root);
        tilt
    }

    /// return: total weight, and total tilt
    pub fn compute_tilt(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => Solution::_compute_tilt(Rc::clone(node)),
        }
    }

    /// leaf nodes return: (val, 0)
    pub fn _compute_tilt(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let node = (*root).borrow();
        let (lft_weight, lft_tilt) = Solution::compute_tilt(&node.left);
        let (rgt_weight, rgt_tilt) = Solution::compute_tilt(&node.right);

        let weight = lft_weight + rgt_weight + node.val;
        let tilt = if lft_weight >= rgt_weight {
            lft_weight - rgt_weight
        } else {
            rgt_weight - lft_weight
        };

        (weight, tilt + lft_tilt + rgt_tilt)
    }
}

fn main() {
    println!("Hello, world!");
}
