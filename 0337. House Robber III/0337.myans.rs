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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // (このノードを盗む場合, このノードを盗まない場合)
        fn dfs (node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            let Some(node) = node else {
                return (0, 0);
            };

            let node = node.borrow();

            let (left_robbed, left_skipped) = dfs(node.left.clone());
            let (right_robbed, right_skipped) = dfs(node.right.clone());

            let robbed = node.val + left_skipped + right_skipped;
            let skipped = left_robbed.max(left_skipped) + right_robbed.max(right_skipped);

            (robbed, skipped)
        }

        let (robbed, skipped) = dfs(root);
        robbed.max(skipped)
    }
}