impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]);

        let mut result = Vec::new();
        let mut current_start = intervals[0][0];
        let mut current_end = intervals[0][1];

        for interval in &intervals[1..] {
            let next_start = interval[0];
            let next_end = interval[1];

            if current_end < next_start {
                result.push(vec![current_start, current_end]);
                current_start = next_start;
                current_end = next_end;
            } else {
                current_end = current_end.max(next_end);
            }
        }
        result.push(vec![current_start, current_end]);

        result
    }
}