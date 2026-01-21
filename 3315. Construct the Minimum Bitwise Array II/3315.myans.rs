impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().map(Self::min_bitwise).collect()
    }

    fn min_bitwise(num: i32) -> i32 {
        if num == 2 {
            return -1;
        }
        let t = (num as u32).trailing_ones();
        num - (1i32 << (t-1))
    }
}