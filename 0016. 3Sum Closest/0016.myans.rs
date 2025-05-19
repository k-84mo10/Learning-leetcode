impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        let mut min_diff = closest_sum.abs_diff(target) as i32;

        for i in 0..nums.len()-2 {
            let (mut left, mut right) = (i+1, nums.len()-1);

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Equal => return sum,
                }

                let diff = sum.abs_diff(target) as i32;
                if diff < min_diff {
                    closest_sum = sum;
                    min_diff = diff;
                }
            }
        }

        closest_sum
    }
}