impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut count = 0;
        let mut current_max = i32::MIN;

        for num in nums {
            // 今まで作った最大値が今作れる最大値と一緒かそれより大きいならパス
            if current_max >= num + k {
                continue;
            } else {
                count += 1;
                current_max = (current_max+1).max(num - k);
            }
        }

        count 
    }
}