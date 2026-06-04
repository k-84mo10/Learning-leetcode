use std::collections::HashMap;

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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();

        fn calc_subtree_sum(
            node: Option<Rc<RefCell<TreeNode>>>,
            freq: &mut HashMap<i32, i32>,
        ) -> i32 {
            let Some(node) = node else {
                return 0;
            };

            let (val, left, right) = {
                let node_r = node.borrow();
                (
                    node_r.val,
                    node_r.left.clone(),
                    node_r.right.clone(),
                )
            };

            let sum =
                val
                + calc_subtree_sum(left, freq)
                + calc_subtree_sum(right, freq);

            *freq.entry(sum).or_insert(0) += 1;

            sum
        }

        calc_subtree_sum(root, &mut freq);

        let max_freq = freq.values().max().copied().unwrap_or(0);
        
        freq.iter()
            .filter(|&(_, &v)| v == max_freq)
            .map(|(&k, _)| k)
            .collect()
    }
}