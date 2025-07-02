impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let mut is_used: Vec<bool> = vec![false; nums.len()];
        let mut cur_permute: Vec<i32> = Vec::new();
        let mut result : Vec<Vec<i32>> = Vec::new();
        Self::dfs(&nums, &mut is_used, &mut cur_permute, &mut result);
        result 
    }

    fn dfs(nums: &Vec<i32>, is_used: &mut Vec<bool>, cur_permute: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if cur_permute.len() == nums.len() {
            result.push(cur_permute.clone());
            return;
        }

        for i in 0..nums.len() {
            if is_used[i] || (i > 0 && nums[i] == nums[i-1] && !is_used[i-1]) { continue; }

            cur_permute.push(nums[i]);
            is_used[i] = true;
            Self::dfs(nums, is_used, cur_permute, result);
            is_used[i] = false;
            cur_permute.pop();
        }
    }
}