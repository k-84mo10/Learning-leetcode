use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut non_distinct_index: Option<usize> = None;

        for i in (0..nums.len()).rev() {
            if set.contains(&nums[i]) {
                non_distinct_index = Some(i);
                break;
            } else {
                set.insert(nums[i]);
            }
        }

        match non_distinct_index {
            None => 0,
            Some(idx) => (idx / 3 + 1) as i32,
        }
    }
}