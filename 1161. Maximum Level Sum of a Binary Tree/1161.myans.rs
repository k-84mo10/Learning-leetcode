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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        q.push_back(root.unwrap());
        let mut level = 1;
        let mut max_sum_level = 1;
        let mut max_sum = i32::MIN;

        while !q.is_empty() {
            let level_size = q.len();
            let mut level_sum = 0;
            for _ in 0..level_size {
                if let Some(node) = q.pop_front() {
                    let node_r = node.borrow();
                    level_sum += node_r.val;
                    if let Some(node_cl) = node_r.left.clone() {
                        q.push_back(node_cl);
                    }       
                    if let Some(node_cr) = node_r.right.clone() {
                        q.push_back(node_cr);
                    }    
                }      
            }
            if level_sum > max_sum {
                max_sum = level_sum;
                max_sum_level = level;
            }
            level += 1;
        }

        max_sum_level
    }
}