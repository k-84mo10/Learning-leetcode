impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut res = 0;
        
        for (idx, &num) in nums.iter().enumerate() {
            let i = (idx + 1) as i32;
            if n % i == 0 {
                res += num * num;
            }
        }

        res
    }
}