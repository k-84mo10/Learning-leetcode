use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 1行で重複が無いか確認
        for i in 0..9 {
            let mut num_set = HashSet::new();
            for j in 0..9 {
                if board[i][j] == '.' { continue; }

                if num_set.contains(&board[i][j]) { return false; }
                num_set.insert(board[i][j]);
            }


            
        }

        // 1列で重複が無いか確認
        for i in 0..9 {
            let mut num_set = HashSet::new();
            for j in 0..9 {
                if board[j][i] == '.' { continue; }

                if num_set.contains(&board[j][i]) { return false; }
                num_set.insert(board[j][i]);
            }
        }

        // 各ボックスで重複が無いか確認
        for i in 0..3 {
            for j in 0..3 {
                if !Self::is_valid_box(&board, i*3, j*3) {
                    return false;
                }
            }
        }


        true
    }

    fn is_valid_box(board: &Vec<Vec<char>>, start_i: usize, start_j: usize) -> bool {
        let mut num_set = HashSet::new();
        for i in start_i..start_i+3 {
            for j in start_j..start_j+3 {
                if board[i][j] == '.' { continue; }

                if num_set.contains(&board[i][j]) { return false; }
                num_set.insert(board[i][j]);
            }
        }
        true
    }
}