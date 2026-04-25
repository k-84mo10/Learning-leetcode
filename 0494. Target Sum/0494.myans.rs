impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();

        if target.abs() > sum {
            return 0;
        }

        if (sum + target) % 2 != 0 {
            return 0;
        }
        
        let plus = ((sum + target) / 2) as usize;

        let mut dp: Vec<i32> = vec![0; plus + 1];
        dp[0] = 1;

        for num in nums {
            let num = num as usize;
            for cur in (num..=plus).rev() {
                dp[cur] += dp[cur - num];
            }
        }

        dp[plus]
    }
}