// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (p, q) = (p?, q?);
        let p_val = p.borrow().val;
        let q_val = q.borrow().val;
        let small = p_val.min(q_val);
        let big = p_val.max(q_val);

        let mut current = root;
        while let Some(node) = current.clone() {
            let node_val = node.borrow().val;
            if small <= node_val && node_val <= big {
                return current;
            } else if node_val < small {
                current = node.borrow().right.clone();
            } else {
                current = node.borrow().left.clone();
            }
        }

        None
    }
}