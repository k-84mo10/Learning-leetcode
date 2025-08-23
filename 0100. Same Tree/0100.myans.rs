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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let (ar, br) = (a.borrow(), b.borrow());
                ar.val == br.val && 
                Self::is_same_tree(ar.left.clone(), br.left.clone()) &&
                Self::is_same_tree(ar.right.clone(), br.right.clone())
            },
            _ => false,
        }
    }
}