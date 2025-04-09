impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut dp: Vec<bool> = vec![false; nums.len()];
        dp[0] = true;

        for i in 0..nums.len() {
            if !dp[i] {
                continue;
            }

            for j in 1..=nums[i] as usize {
                if i+j >= nums.len() {
                    break;
                }
                dp[i+j] = true;
            }
        }
        dp[nums.len()-1]
    }
}