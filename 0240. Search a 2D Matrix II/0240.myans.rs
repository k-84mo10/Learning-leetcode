impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let (mut current_i, mut current_j) = (0, n-1);

        while current_i < m {
            let current = matrix[current_i][current_j];
            if current == target {
                return true;
            } else if current < target {
                current_i += 1;
            } else {
                if current_j == 0 { break; }
                current_j -= 1;
            }
        }

        false
    }
}