impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let (new_start, new_end) = (new_interval[0], new_interval[1]);

        let mut i = 0;
        let mut result = Vec::new();

        while i < n && intervals[i][1] < new_start {
            result.push(intervals[i].clone());
            i += 1;
        }

        let mut insert_start = new_start;
        let mut insert_end = new_end;
        while i < n && intervals[i][0] <= new_end {
            insert_start = insert_start.min(intervals[i][0]);
            insert_end = insert_end.max(intervals[i][1]);
            i += 1;
        }
        result.push(vec![insert_start, insert_end]);

        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}