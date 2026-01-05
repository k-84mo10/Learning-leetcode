impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut abs_sum: i64 = 0;
        let mut neg_cnt: i32 = 0;
        let mut min_abs: i64 = i64::MAX;

        for row in &matrix {
            for &num in row {
                let n = num as i64;
                if n < 0 { neg_cnt += 1; }
                let a = n.abs();
                abs_sum += a;
                min_abs = min_abs.min(a);
            }
        }

        if neg_cnt & 1 == 1 {
            abs_sum - 2 * min_abs
        } else {
            abs_sum
        }
    }
}