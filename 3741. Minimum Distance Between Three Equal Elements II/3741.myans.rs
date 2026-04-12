use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut index_map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut half_cur_min_distance = i32::MAX;

        for (idx, &num) in nums.iter().enumerate() {
            let idx_i = idx as i32;
            if let Some((first, second)) = index_map.get_mut(&num) {
                if *first != -1 {
                    half_cur_min_distance = half_cur_min_distance.min(idx_i - *first);
                } 
                *first = *second;
                *second = idx_i;
            } else {
                index_map.insert(num, (-1, idx_i));
            }
        }

        if half_cur_min_distance == i32::MAX { -1 } else { 2 * half_cur_min_distance } 
    }
}