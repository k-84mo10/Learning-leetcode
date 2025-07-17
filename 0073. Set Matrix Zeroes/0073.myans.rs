use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut m_set = HashSet::new();
        let mut n_set = HashSet::new();

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    m_set.insert(i);
                    n_set.insert(j);
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if m_set.contains(&i) || n_set.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}