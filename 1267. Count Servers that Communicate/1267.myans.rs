impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut row_count: Vec<i32> = vec![0; rows];
        let mut col_count: Vec<i32> = vec![0; cols];

        let mut positions: Vec<(usize, usize)> = Vec::new();
        let mut communicatable = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                    positions.push((i, j));
                }
            }
        }

        for position in positions {
            let (i, j) = (position.0, position.1);
            if row_count[i] > 1 || col_count[j] > 1 {
                communicatable += 1;
            }
        }

        communicatable
    }
}