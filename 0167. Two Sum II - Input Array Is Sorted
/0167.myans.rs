impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len()-1);

        while left < right {
            let current_sum = numbers[left] + numbers[right];

            if current_sum == target {
                return vec![(left+1) as i32, (right+1) as i32];
            } else if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        Vec::new()
    }
}