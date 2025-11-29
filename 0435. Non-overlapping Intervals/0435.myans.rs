impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[1]);

        let mut removed = 0;
        let mut last_end = intervals[0][1];

        for interval in intervals.iter().skip(1) {
            let start = interval[0];
            let end = interval[1];

            if start < last_end {
                removed += 1;
            } else {
                last_end = end;
            }
        }

        removed
    }
}