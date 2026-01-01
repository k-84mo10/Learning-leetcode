use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut last: HashMap<i32, usize> = HashMap::new();
        let k = k as usize;

        for (idx, &num) in nums.iter().enumerate() {
            if let Some(prev) = last.insert(num, idx) {
                if idx - prev <= k {
                    return true;
                }
            }
        }

        false
    }
}