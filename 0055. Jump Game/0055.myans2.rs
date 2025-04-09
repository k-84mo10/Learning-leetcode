impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut far_distance = 0;

        for i in 0..nums.len()-1 {
            if far_distance < i {
                return false;
            }
            
            far_distance = far_distance.max(i+(nums[i] as usize));
            if far_distance >= nums.len()-1 {
                return true;
            }
        }
        false
    }
}