impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut cur_max, mut cur_min) = (nums[0], nums[0]);
        let (mut max_sum, mut min_sum) = (nums[0], nums[0]);
        let mut total_sum = nums[0];

        for &num in nums.iter().skip(1) {
            cur_max = num.max(cur_max + num);
            max_sum = max_sum.max(cur_max);

            cur_min = num.min(cur_min + num);
            min_sum = min_sum.min(cur_min);

            total_sum += num;
        }

        // 負の数の要素しかない場合の処理
        if max_sum < 0 {
            return max_sum;
        }

        // 配列全体から最小和の部分配列を抜いた循環配列の和
        let wrap_sum = total_sum - min_sum;

        max_sum.max(wrap_sum)
    }
}