impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, index: usize, current_xor: i32) -> i32{
            if index == nums.len() {
                return current_xor;
            }

            let with_element = dfs(&nums, index+1, current_xor^nums[index]);
            let without_element = dfs(&nums, index+1, current_xor);

            with_element + without_element
        }

        dfs(&nums, 0, 0)
    }
}