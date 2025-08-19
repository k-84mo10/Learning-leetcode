impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut answer = 0;

        for x in nums.iter() {
            if *x == 0 {
                count += 1;
                answer += count;
            } else {
                count = 0;
            }
        }

        answer
    }
}