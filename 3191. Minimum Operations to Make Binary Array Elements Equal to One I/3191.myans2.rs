impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut flip_one_ahead = false;
        let mut flip_two_ahead = false;
        let mut count = 0;

        for x in nums {
            if (x == 0) ^ flip_one_ahead {
                count += 1;
                (flip_one_ahead, flip_two_ahead) = (!flip_two_ahead, true);
            } else {
                (flip_one_ahead, flip_two_ahead) = (flip_two_ahead, false);
            }
        }

        if (flip_one_ahead, flip_two_ahead) != (false, false) {
            return -1;
        } 
        count 
    }
}
