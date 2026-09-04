impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_max: Vec<i32> = nums
            .iter()
            .scan(i32::MIN, |max_val, &x| {
                *max_val = (*max_val).max(x);
                Some(*max_val)
            })
            .collect();

        let mut suffix_min: Vec<i32> = nums
            .iter()
            .rev()
            .scan(i32::MAX, |min_val, &x| {
                *min_val = (*min_val).min(x);
                Some(*min_val)
            })
            .collect();
        suffix_min.reverse();

        prefix_max
            .iter()
            .zip(suffix_min)
            .position(|(&pmax, smin)| pmax - smin <= k)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}