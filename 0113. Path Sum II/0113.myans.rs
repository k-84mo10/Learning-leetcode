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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut answer_paths: Vec<Vec<i32>> = Vec::new();

        Self::dfs(root, target_sum, 0, &mut Vec::new(), &mut answer_paths);
        answer_paths
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>, 
        target_sum: i32, 
        previous_sum: i32, 
        path: &mut Vec<i32>, 
        answer_paths: &mut Vec<Vec<i32>>
    ) {
        let Some(nrc) = node else { return; };

        let (val, left, right) = {
            let nref = nrc.borrow();
            (nref.val, nref.left.clone(), nref.right.clone()) 
        };

        let current_sum = previous_sum + val;
        path.push(val);

        if left.is_none() && right.is_none() && current_sum == target_sum {
            answer_paths.push(path.clone());
        }

        if left.is_some() {
            Self::dfs(left, target_sum, current_sum, path, answer_paths);
        }

        if right.is_some() {
            Self::dfs(right, target_sum, current_sum, path, answer_paths);
        }

        path.pop();
    }
}