impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut first_min = i32::MAX;
        let mut second_min = i32::MAX;

        for &num in nums.iter().skip(1) {
            if num < first_min {
                second_min = first_min;
                first_min = num;
            } else if num < second_min {
                second_min = num;
            }
        }

        nums[0] + first_min + second_min
    }
}