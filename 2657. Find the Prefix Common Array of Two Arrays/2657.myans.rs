use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut exists: HashSet<i32> = HashSet::new();

        let mut common = 0;
        let mut prefix_common_array = vec![0; a.len()];

        for ((i, &x), &y) in a.iter().enumerate().zip(b.iter()) {
            if !exists.insert(x) {
                common += 1;
            }

            if !exists.insert(y) {
                common += 1;
            }

            prefix_common_array[i] = common;
        }

        prefix_common_array 
    }
}