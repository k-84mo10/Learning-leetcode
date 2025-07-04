impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut bottom) = (0, matrix.len()-1);
        let (mut left, mut right) = (0, matrix[0].len()-1);
        let mut result = Vec::new();

        while left <= right && top <= bottom {
            for j in left..=right {
                result.push(matrix[top][j]);
            }
            top += 1;
            if top > bottom { break; }

            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            if right == 0 { break; }
            right -= 1;
            if left > right { break; }

            for j in (left..=right).rev() {
                result.push(matrix[bottom][j]);
            }
            if bottom == 0 { break; }
            bottom -= 1;
            if top > bottom { break; }

            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }

        result
    }
}