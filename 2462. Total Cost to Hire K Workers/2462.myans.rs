use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let n = costs.len();
        let candidates = candidates as usize;

        let mut first_candidates_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut last_candidates_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let (mut left, mut right) = (0, n-1);
        
        while left < candidates {
            first_candidates_heap.push(Reverse(costs[left]));
            left += 1;
        }

        while left <= right && n-1-right < candidates {
            last_candidates_heap.push(Reverse(costs[right]));
            right -= 1;
        }

        let mut total_cost = 0i64;

        for _ in 0..k {
            let first = first_candidates_heap.peek().copied();
            let last = last_candidates_heap.peek().copied();

            match (first, last) {
                (None, None) => {
                    break;
                }
                (Some(Reverse(f)), None) => {
                    total_cost += f as i64;
                    first_candidates_heap.pop();
                }
                (None, Some(Reverse(l))) => {
                    total_cost += l as i64;
                    last_candidates_heap.pop();
                }
                (Some(Reverse(f)), Some(Reverse(l))) => {
                    if f <= l {
                        total_cost += f as i64;
                        first_candidates_heap.pop();
                        if left <= right {
                            first_candidates_heap.push(Reverse(costs[left]));
                            left += 1;
                        }
                    } else {
                        total_cost += l as i64;
                        last_candidates_heap.pop();
                        if left <= right {
                            last_candidates_heap.push(Reverse(costs[right]));
                            right -= 1;
                        }   
                    }
                }
            }
        }
        total_cost
    }
}