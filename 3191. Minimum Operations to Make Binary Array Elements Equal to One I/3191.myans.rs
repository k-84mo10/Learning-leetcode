impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0;

        for i in 0..nums.len()-2 {
            if nums[i] == 0 {
                nums[i..i+3].iter_mut().for_each(|x| *x ^= 1);
                count += 1;
            }
        }

        if nums.iter().rev().take(2).any(|&x| x == 0) {
            return -1;
        }
        count
    }
}
