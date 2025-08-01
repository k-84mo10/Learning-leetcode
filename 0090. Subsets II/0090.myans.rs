impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = Vec::new();
        let mut result = Vec::new();

        let mut nums = nums;
        nums.sort();

        Self::dfs(&nums, &mut current, &mut result, 0);

        result
    }

    fn dfs(nums: &Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, index: usize) {
        result.push(current.clone());

        for i in index..nums.len() {
            if i > index && nums[i] == nums[i-1] {
                continue;
            }

            current.push(nums[i]);
            Self::dfs(nums, current, result, i+1);
            current.pop();
        }
    }
}