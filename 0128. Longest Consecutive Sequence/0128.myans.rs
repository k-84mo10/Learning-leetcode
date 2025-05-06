use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set:HashSet<i32> = nums.into_iter().collect();
        let mut longest_stream = 0;

        for &num in &set {
            if set.contains(&(num-1)) {
                continue;
            }
            let mut current_stream = 1;
            let mut current_num = num + 1;
            while set.contains(&current_num) {
                current_stream += 1;
                current_num += 1;
            }
            longest_stream = longest_stream.max(current_stream);
        }
        longest_stream
    }
}