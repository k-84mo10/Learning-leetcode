impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let mut answer: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i-1] { continue; }
            if nums[i] > 0 { break; }
            
            let object = -1 * nums[i];
            let (mut left, mut right) = (i+1, nums.len()-1);

            while left < right {
                let sum = nums[left] + nums[right];
                if sum == object {
                    answer.push(vec![nums[i], nums[left], nums[right]]);
                }
                if sum <= object {
                    while left < right && nums[left] == nums[left+1] {
                        left += 1;
                    }
                    left += 1;
                } 
                if sum >= object {
                    while left < right && nums[right] == nums[right-1] {
                        right -= 1;
                    }
                    right -= 1;
                }
            }
        }
        answer
    }
}