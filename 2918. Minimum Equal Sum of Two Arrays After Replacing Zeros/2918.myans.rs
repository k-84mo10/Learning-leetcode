impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut nums1_sum = 0i64;
        let mut nums1_zero_counter = 0i64;
        let mut nums2_sum = 0i64;
        let mut nums2_zero_counter = 0i64;

        for &x in &nums1 {
            nums1_sum += x as i64;
            if x == 0 { 
                nums1_zero_counter += 1; 
                nums1_sum += 1;
            }
        }
        for &x in &nums2 {
            nums2_sum += x as i64;
            if x == 0 { 
                nums2_zero_counter += 1; 
                nums2_sum += 1;
            }
        }


        if nums1_zero_counter == 0 && nums1_sum < nums2_sum { return -1 };
        if nums2_zero_counter == 0 && nums2_sum < nums1_sum { return -1 };

        nums1_sum.max(nums2_sum)
    }
}