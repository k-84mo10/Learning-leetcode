use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();

        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (&num, &count) in freq.iter() {
            heap.push(Reverse((count, num)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_iter().map(|Reverse((_, num))| num).collect()
    }
}