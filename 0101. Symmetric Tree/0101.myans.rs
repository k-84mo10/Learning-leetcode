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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(root) {
            None => true,
            Some(root) => {
                let root_r = root.borrow();
                Self::dfs(root_r.left.clone(), root_r.right.clone())
            },
            _ => false,
        }
    }

    fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match((p, q)) {
            (None, None) => true,
            (Some(a), (Some(b))) => {
                let (ar, br) = (a.borrow(), b.borrow());
                ar.val == br.val && 
                Self::dfs(ar.left.clone(), br.right.clone()) &&
                Self::dfs(ar.right.clone(), br.left.clone())
            },
            _ => false,
        }
    }
}