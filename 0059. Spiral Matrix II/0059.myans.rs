impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];

        let mut i = 1;
        let total = (n * n) as i32;

        let (mut left, mut right) = (0, n-1);
        let (mut top, mut bottom) = (0, n-1);

        loop {
            // 上段
            for j in left..=right {
                matrix[top][j] = i;
                i += 1;
                if i > total { break; }
            }
            top += 1;

            // 右端
            for j in top..=bottom {
                matrix[j][right] = i;
                i += 1;
                if i > total { break; }
            }
            if right == 0 { break; }
            right -= 1;

            // 下段
            for j in (left..=right).rev() {
                matrix[bottom][j] = i;
                i += 1;
                if i > total { break; }
            }
            if bottom == 0 { break; }
            bottom -= 1;

            // 左端
            for j in (top..=bottom).rev() {
                matrix[j][left] = i;
                i += 1;
                if i > total { break; }
            }
            left += 1;
        }

        matrix
    }
}