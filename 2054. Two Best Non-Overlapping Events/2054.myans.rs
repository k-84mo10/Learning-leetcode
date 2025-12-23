impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let events_num = events.len();

        let mut events: Vec<(i32, i32, i32)> = events
            .into_iter()
            .map(|e| (e[0], e[1], e[2]))
            .collect();

        events.sort_unstable_by_key(|&(_, end, _)| end);

        let mut prefix_max: Vec<i32> = vec![0; events_num];
        prefix_max[0] = events[0].2;
        for i in 1..events_num {
            prefix_max[i] = prefix_max[i-1].max(events[i].2);
        }

        let mut answer = prefix_max[events_num - 1];

        for i in 0..events_num {
            let (start, _end, value) = events[i];

            let pos = lower_bound_end_ge(&events, i, start);
            let best_first = if pos == 0 {
                0
            } else {
                prefix_max[pos - 1]
            };

            let cand = value + best_first;
            answer = answer.max(cand);
        }

        answer
    }
}

fn lower_bound_end_ge(events: &[(i32, i32, i32)], hi: usize, target:i32) -> usize {
    let mut left = 0usize;
    let mut right = hi;
    while left < right {
        let middle = (left + right) / 2;
        if events[middle].1 >= target {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    left
}