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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }

        fn calc_heights(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if node.is_none() { return 0; }

            let node = node.as_ref().unwrap().borrow();
            let mut height = 1;
            let mut search = node.left.clone();

            while search.is_some() {
                let next = search.as_ref().unwrap().borrow().left.clone();
                height += 1;
                search = next;
            }
            
            height
        }    

        let mut num = 0;
        let mut node = root;
        while node.is_some() {
            let left = node.as_ref().unwrap().borrow().left.clone();
            let right = node.as_ref().unwrap().borrow().right.clone();

            let left_height = calc_heights(left.clone());
            let right_height = calc_heights(right.clone());

            if left_height == right_height {
                // 左は完全二分木
                num += 1 << left_height;
                node = right;
            } else {
                // 右は完全二分木
                num += 1 << right_height;
                node = left;
            }
        }

        num
    }
}