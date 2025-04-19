use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut pairs: Vec<(i32,i32)> = nums2.into_iter().zip(nums1.into_iter()).collect();
        pairs.sort_by(|a, b| b.0.cmp(&a.0));
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut nums1_sum = 0i64;
        let mut max_score = 0i64;

        for &(min2, val1) in pairs.iter() {
            heap.push(Reverse(val1));
            nums1_sum += val1 as i64;

            if heap.len() > k {
                if let Some(Reverse(smallest)) = heap.pop() {
                    nums1_sum -= smallest as i64;
                }
            }

            if heap.len() == k {
                max_score = max_score.max(nums1_sum*min2 as i64);
            }
        }

        max_score
    }
}