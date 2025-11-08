impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows_size = board.len();
        if rows_size == 0 {
            return;
        }
        let cols_size = board[0].len();

        // 1. 外周にある 'O' から DFS して、外周とつながっている 'O' を一時的に '1' でマークする
        for col in 0..cols_size {
            // 上端
            if board[0][col] == 'O' {
                board[0][col] = '1';
                Self::dfs(board, 0, col);
            }
            // 下端
            if board[rows_size - 1][col] == 'O' {
                board[rows_size - 1][col] = '1';
                Self::dfs(board, rows_size - 1, col);
            }
        }
        for row in 0..rows_size {
            // 左端
            if board[row][0] == 'O' {
                board[row][0] = '1';
                Self::dfs(board, row, 0);
            }
            // 右端
            if board[row][cols_size - 1] == 'O' {
                board[row][cols_size - 1] = '1';
                Self::dfs(board, row, cols_size - 1);
            }
        }

        // 2. 残っている 'O' は囲まれているので 'X' に、
        //    一時マークの '1' は元に戻して 'O' にする
        for row in 0..rows_size {
            for col in 0..cols_size {
                match board[row][col] {
                    'O' => board[row][col] = 'X',
                    '1' => board[row][col] = 'O',
                    _ => {}
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
        let rows_size = board.len();
        let cols_size = board[0].len();

        // 上
        if row > 0 && board[row - 1][col] == 'O' {
            board[row - 1][col] = '1';
            Self::dfs(board, row - 1, col);
        }
        // 下
        if row + 1 < rows_size && board[row + 1][col] == 'O' {
            board[row + 1][col] = '1';
            Self::dfs(board, row + 1, col);
        }
        // 左
        if col > 0 && board[row][col - 1] == 'O' {
            board[row][col - 1] = '1';
            Self::dfs(board, row, col - 1);
        }
        // 右
        if col + 1 < cols_size && board[row][col + 1] == 'O' {
            board[row][col + 1] = '1';
            Self::dfs(board, row, col + 1);
        }
    }
}
