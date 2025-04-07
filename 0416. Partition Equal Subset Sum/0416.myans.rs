impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();

        if sum % 2 != 0 {
            return false;
        }

        let target = (sum / 2) as usize;
        let mut dp: Vec<bool> = vec![false; target+1];
        dp[0] = true;

        for num in &nums {
            let num = *num as usize;
            for i in (num..=target).rev() {
                if dp[i-num] {
                    if i == target {
                        return true;
                    }
                    dp[i] = true;
                } 
            }
        }
        false
    }
}