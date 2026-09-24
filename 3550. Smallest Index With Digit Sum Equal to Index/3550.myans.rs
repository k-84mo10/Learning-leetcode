impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (idx, &num) in nums.iter().enumerate() {
            let idx = idx as i32;
            let mut num = num;
            let mut sum_digit = 0;
            while num > 0 {
                sum_digit += num % 10;
                num /= 10;
            }
            if idx == sum_digit {
                return idx;
            }
        }
        -1
    }
}