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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height_or_unbalanced(&root) != -1
    }

    fn height_or_unbalanced(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(rc) = node else {
            return 0
        };

        let (l, r) = {
            let rc_r = rc.borrow();
            (rc_r.left.clone(), rc_r.right.clone())
        };

        let hl = Self::height_or_unbalanced(&l);
        if hl == -1 { return -1; }

        let hr = Self::height_or_unbalanced(&r);
        if hr == -1 { return -1; }

        if (hl - hr).abs() > 1 {
            -1
        } else {
            1 + hl.max(hr)
        }
    }
}