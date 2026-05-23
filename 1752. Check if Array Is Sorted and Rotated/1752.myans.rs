impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut decreases = 0;

        for w in nums.windows(2) {
            if w[0] > w[1] {
                decreases += 1;
            }
        }

        if nums[0] < nums[nums.len()-1] {
            decreases += 1;
        }

        decreases < 2
    }
}