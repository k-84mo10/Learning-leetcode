impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut usedbit = 0;
        let mut left = 0;
        let mut right = 0;
        let mut longest_window = 1;

        while right < nums.len() {
            // bitの重複が無いか確認
            while (usedbit & nums[right] != 0) {
                usedbit ^= nums[left];
                left += 1;
            }

            usedbit |= nums[right];

            longest_window = std::cmp::max(longest_window, right-left+1);
            right += 1;
        }

        longest_window as i32
    }
}