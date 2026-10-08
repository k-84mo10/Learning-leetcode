impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev = vec![None; n];
        let mut max_idx = 0;

        let mut nums = nums;
        nums.sort_unstable();

        for (i, &num) in nums.iter().enumerate() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    prev[i] = Some(j);
                }
            }


            if dp[i] > dp[max_idx] {
                max_idx = i;
            }
        }

        let mut idx = Some(max_idx);
        let mut ans = Vec::new();
        while let Some(i) = idx {
            ans.push(nums[i]);
            idx = prev[i];
        }

        ans
    }
}