impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];

        for row in 0..m {
            for col in 0..n {
                if visited[row][col] {
                    continue;
                }

                if Self::dfs(&grid, None, row, col, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(grid: &Vec<Vec<char>>, parent: Option<(usize, usize)>, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        visited[row][col] = true;

        let (m, n) = (grid.len(), grid[0].len());
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dr, dc) in dirs {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if grid[nr][nc] != grid[row][col] {
                continue;
            }

            if Some((nr, nc)) == parent {
                continue;
            }

            if visited[nr][nc] {
                return true;
            }

            if Self::dfs(grid, Some((row, col)), nr, nc, visited) {
                return true;
            }
        }

        false
    }
}