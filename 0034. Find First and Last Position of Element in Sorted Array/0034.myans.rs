impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 配列が無い場合
        if nums.is_empty() {
            return vec![-1, -1];
        }

        // targetの始点を探索
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // targetが存在するか確認
        if left == nums.len() || nums[left] != target {
            return vec![-1, -1];
        }

        let start = left;

        // targetの終点を探索
        right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let end = right - 1;
        vec![start as i32, end as i32]
    }
}