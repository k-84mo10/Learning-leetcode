impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return true;
            }

            // 左・中央・右が同じなら範囲を縮小して曖昧さを避ける
            if nums[left] == nums[mid] && nums[mid] == nums[right] {
                left += 1;
                if right == 0 { break; } // usize の underflow を避ける
                right -= 1;
            }
            // 左側がソートされている
            else if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            // 右側がソートされている
            else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        false
    }
}