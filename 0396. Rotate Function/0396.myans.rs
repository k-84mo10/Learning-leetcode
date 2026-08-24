impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();

        let mut f: i64 = nums.iter()
            .enumerate()
            .map(|(i, &x)| i as i64 * x as i64)
            .sum();

        let mut ans = f;

        for k in 1..nums.len() {
            f += sum - n * nums[nums.len() - k] as i64;
            ans = ans.max(f);
        }

        ans as i32
    }
}