impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut idx1 = 0usize;
        let mut idx2 = 0usize;

        while idx1 < len1 && idx2 < len2 {
            let n1 = nums1[idx1];
            let n2 = nums2[idx2];
            if n1 == n2 {
                return n1;
            } else if n1 < n2 {
                idx1 += 1;
            } else {
                idx2 += 1;
            }
        }

        -1
    }
}