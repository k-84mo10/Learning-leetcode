impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0 as usize;
        let mut right = 0 as usize;

        while right < nums.len() {
            if nums[right] != 0 {
                nums[left] = nums[right];
                left += 1;
            }
            right += 1;
        }

        while left != right {
            nums[left] = 0;
            left += 1;
        }
    }
}