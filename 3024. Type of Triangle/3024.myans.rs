impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums.clone();
        nums.sort();

        if nums[0] + nums[1] <= nums[2] { return "none".to_string(); }
        if nums[0] != nums[1] && nums[1] != nums[2] { return "scalene".to_string(); }
        if nums[0] == nums[2] { return "equilateral".to_string(); }
        "isosceles".to_string()
    }
}