impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut one, mut two) = (0, 0);

        for num in nums {
            one = (one ^ num) & !two;
            two = (two ^ num) & !one;
        }

        one
    }
}