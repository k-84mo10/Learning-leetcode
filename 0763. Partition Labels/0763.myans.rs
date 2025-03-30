use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut char_end_hash = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            char_end_hash.insert(c, i);
        }

        let mut partition_start = 0;
        let mut partition_end = 0;
        let mut partition: Vec<i32> = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if let Some(&char_end) = char_end_hash.get(&c) {
                partition_end = partition_end.max(char_end);
            }

            if i == partition_end {
                partition.push((partition_end - partition_start + 1) as i32);
                partition_start = i + 1;
            }
        }
        partition
    }
}
