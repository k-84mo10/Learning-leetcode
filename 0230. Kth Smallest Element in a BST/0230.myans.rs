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
use std::collections::BinaryHeap;
use std::collections::VecDeque;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        q.push_back(root.clone());

        while let Some(node_opt) = q.pop_front() {
            if let Some(node_rc) = node_opt {
                let node = node_rc.borrow();
                pq.push(node.val);
                if pq.len() > (k as usize) {
                    pq.pop();
                }
                q.push_back(node.left.clone());
                q.push_back(node.right.clone());
            }
        }
        *pq.peek().unwrap()
    }
}