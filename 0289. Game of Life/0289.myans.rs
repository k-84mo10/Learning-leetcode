impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len();
        let cols = board[0].len();

        let prefix_sum = Self::create_prefix_sum(board.to_vec());

        for row in 0..rows {
            for col in 0..cols {
                let surround_alive_cells 
                    = prefix_sum[row+3][col+3] 
                    - prefix_sum[row][col+3]
                    - prefix_sum[row+3][col]
                    + prefix_sum[row][col]
                    - board[row][col];
                if board[row][col] == 1 {
                    board[row][col] = match surround_alive_cells {
                        2 | 3 => 1,
                        _ => 0,
                    };
                } else {
                    board[row][col] = match surround_alive_cells {
                        3 => 1,
                        _ => 0,
                    };
                }
            }
        }
    }

    fn create_prefix_sum(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = board.len();
        let cols = board[0].len();

        let mut prefix_sum: Vec<Vec<i32>> = vec![vec![0; cols+3]; rows+3];
        
        for row in 0..rows {
            for col in 0..cols {
                prefix_sum[row+2][col+2] = prefix_sum[row+2][col+1] + board[row][col];
            }
            prefix_sum[row+2][cols+2] = prefix_sum[row+2][cols+1];
        }

        for row in 0..rows+1 {
            for col in 0..cols+1 {
                prefix_sum[row+2][col+2] += prefix_sum[row+1][col+2];
            }
        }

        prefix_sum
    }
}