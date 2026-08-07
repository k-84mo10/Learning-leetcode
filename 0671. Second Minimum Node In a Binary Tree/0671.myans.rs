
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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min_val: i32) -> Option<i32> {
            let node = node?;
            let node = node.borrow();

            if node.val > min_val {
                return Some(node.val);
            }

            match (
                dfs(node.left.clone(), min_val),
                dfs(node.right.clone(), min_val),
            ) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) | (None, Some(a)) => Some(a),
                _ => None,
            }
        }

        let root = root.unwrap();
        let min_val = root.borrow().val;

        dfs(Some(root), min_val).unwrap_or(-1)
    }
}