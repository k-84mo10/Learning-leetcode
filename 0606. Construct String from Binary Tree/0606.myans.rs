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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = String::new();
        if let Some(node) = root {
            Self::dfs(&node, &mut out);
        }
        out
    }

    fn dfs(node: &Rc<RefCell<TreeNode>>, out: &mut String) {
        let (val, left, right) = {
            let n = node.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        out.push_str(&val.to_string());

        match (left, right) {
            (None, None) => {}
            (Some(l), None) => {
                out.push('(');
                Self::dfs(&l, out);
                out.push(')');
            }
            (None, Some(r)) => {
                out.push_str("()(");
                Self::dfs(&r, out);
                out.push(')');
            }
            (Some(l), Some(r)) => {
                out.push('(');
                Self::dfs(&l, out);
                out.push_str(")(");
                Self::dfs(&r, out);
                out.push(')');
            }
        }
    }
}