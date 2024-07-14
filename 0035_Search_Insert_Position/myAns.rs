impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let nums_size = nums.len()-1;
        let mut left = 0;
        let mut right = nums_size;

        while left <= right {
            let medium = left + (right-left)/2;
            if (nums[medium] == target) {
                return medium as i32;
            } else if (nums[medium] < target) {
                left = medium + 1;
            } else {
                if medium == 0 {
                    break;
                }
                right = medium - 1;
            }
        }

        left as i32
    }
}