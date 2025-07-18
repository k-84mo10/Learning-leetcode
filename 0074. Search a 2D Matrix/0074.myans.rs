impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if target < matrix[0][0] { return false; }

        let m = matrix.len();
        let n = matrix[0].len();
        
        // 縦で2分探索
        let (mut left, mut right) = (0, m);

        while left < right {
            let mid = (left + right) / 2;
            if matrix[mid][0] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let row = left - 1;
        if matrix[row][n-1] < target { return false; }

        // 横で2分探索
        (left, right) = (0, n);
        while left <= right {
            let mid = (left + right) / 2;
            if matrix[row][mid] == target {
                return true;
            } else if matrix[row][mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}