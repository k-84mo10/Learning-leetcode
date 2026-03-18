impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cols = grid[0].len();
        let mut col_sums = vec![0; cols];

        let mut answer = 0;

        for row in &grid {
            let mut current = 0;

            for (j, &val) in row.iter().enumerate() {
                col_sums[j] += val;
                current += col_sums[j];

                if current <= k {
                    answer += 1;
                } else {
                    if j == 0 {
                        return answer;
                    }
                    break;
                }
            }
        }

        answer
    }
}