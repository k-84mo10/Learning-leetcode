use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut num_groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, &num) in nums.iter().enumerate() {
            num_groups.entry(num).or_default().push(idx);
        }

        let mut ans = vec![0i64; nums.len()];

        for indices in num_groups.values() {
            let m = indices.len();
            if m == 1 {
                continue;
            }

            let mut prefix = vec![0i64; m + 1];
            for i in 0..m {
                prefix[i+1] = prefix[i] + indices[i] as i64;
            }

            for i in 0..m {
                let idx = indices[i] as i64;

                let left_count = i as i64;
                let left_sum = prefix[i];
                let left_dist = idx * left_count - left_sum;

                let right_count = (m - i - 1) as i64;
                let right_sum = prefix[m] - prefix[i+1];
                let right_dist = right_sum - idx * right_count;

                ans[indices[i]] = left_dist + right_dist;
            }
        }

        ans
    }
}