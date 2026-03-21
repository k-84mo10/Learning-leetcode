impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = grid.clone();
        let (x, y, k) = (x as usize, y as usize, k as usize);

        for i in 0..k {
            let src = &grid[x+k-i-1][y..y+k];
            let dst = &mut ans[x+i][y..y+k];
            dst.clone_from_slice(src);
        }

        ans
    }
}