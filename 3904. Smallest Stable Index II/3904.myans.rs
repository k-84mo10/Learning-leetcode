impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_max: Vec<i32> = nums
            .iter()
            .scan(i32::MIN, |cur_max, &x| {
                *cur_max = (*cur_max).max(x);
                Some(*cur_max)
            })
            .collect();

        let mut suffix_min: Vec<i32> = nums
            .iter()
            .rev()
            .scan(i32::MAX, |cur_min, &x| {
                *cur_min = (*cur_min).min(x);
                Some(*cur_min)
            })
            .collect();

        suffix_min.reverse();

        prefix_max.iter()
            .zip(suffix_min)
            .position(|(&pmax, smin)| pmax - smin <= k)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}