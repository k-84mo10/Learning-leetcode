impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = (k as usize) % n;
        Self::reverse(nums, 0, n-1);
        if k > 0 {
            Self::reverse(nums, 0, k-1);
        }
        Self::reverse(nums, k, n-1);
    }

    fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}