impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums: Vec<i32> = grid.into_iter().flatten().collect();

        let rem = nums[0] % x;
        for &v in &nums {
            if v % x != rem {
                return -1;
            }
        }

        nums.sort();

        let median = nums[nums.len() / 2];
        let mut ans = 0;

        for &v in &nums {
            ans += (v - median).abs() / x;
        }

        ans
    }
}