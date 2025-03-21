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
use std::collections::VecDeque;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut height = 1;

        q.push_back(root.unwrap());

        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let node_ref = node.borrow();

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return height;
                }

                if let Some(left) = node_ref.left.clone() {
                    q.push_back(left);
                }

                if let Some(right) = node_ref.right.clone() {
                    q.push_back(right);
                }
            }
            height += 1;
        }
        height
    }
}