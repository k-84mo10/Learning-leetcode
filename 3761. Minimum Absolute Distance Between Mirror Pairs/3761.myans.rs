use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut index_map: HashMap<i32, i32> = HashMap::new();
        let mut min_distance = i32::MAX;

        for (idx, &num) in nums.iter().enumerate() {
            let idx = idx as i32;
            if let Some(reverse_idx) = index_map.get(&num) {
                let distance = idx - reverse_idx;
                min_distance = min_distance.min(distance);
            } 
            index_map.insert(Self::reverse_num(num), idx);
        }

        if min_distance == i32::MAX {-1} else {min_distance}
    }

    fn reverse_num(num: i32) -> i32 {
        let mut num = num;
        let mut ans = 0;

        while num != 0 {
            let cur = num % 10;
            num /= 10;
            ans *= 10;
            ans += cur;
        }

        ans
    }
}