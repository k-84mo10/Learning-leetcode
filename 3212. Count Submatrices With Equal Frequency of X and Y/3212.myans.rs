impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut cols = grid[0].len();
        let mut col_sums = vec![(0i32, 0i32); cols];
        let mut answer = 0;

        for row in &grid {
            let mut current = (0i32, 0i32);

            for (j, &s) in row.iter().enumerate() {
                match s {
                    'X' => current.0 += 1,
                    'Y' => current.1 += 1,
                    _ => {} 
                };

                col_sums[j].0 += current.0;
                col_sums[j].1 += current.1;

                if col_sums[j].0 > 0 && col_sums[j].0 == col_sums[j].1 {
                    answer += 1;
                }
            }
        }

        answer
    }
}