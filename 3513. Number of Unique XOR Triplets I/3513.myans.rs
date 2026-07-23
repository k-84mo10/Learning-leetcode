impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n <= 2 {
            n 
        } else {
            ((n+1) as u32).next_power_of_two() as i32
        }
    }
}