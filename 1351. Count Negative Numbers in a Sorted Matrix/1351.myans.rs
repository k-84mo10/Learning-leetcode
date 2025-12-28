impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut negatives = 0;
        let mut cur_row = 0;
        let mut cur_col = cols - 1;

        while cur_row < rows && cur_col < cols {
            if grid[cur_row][cur_col] >= 0 {
                negatives += (cols - cur_col - 1) as i32;
                cur_row += 1;
            } else {
                cur_col -= 1;
            }
        } 

        negatives += ((rows - cur_row) * cols) as i32;
        negatives
    }
}