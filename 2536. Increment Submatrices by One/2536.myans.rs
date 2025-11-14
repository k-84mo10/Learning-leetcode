impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff: Vec<Vec<i32>> = vec![vec![0; n+1]; n+1];

        for query in &queries {
            let [row1, col1, row2, col2] = <[i32; 4]>::try_from(&query[..]).unwrap();
            let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);

            diff[row1][col1] += 1;
            diff[row1][col2+1] -= 1;
            diff[row2+1][col1] -= 1;
            diff[row2+1][col2+1] += 1;
        }

        for i in 1..n {
            for j in 0..n {
                diff[i][j] += diff[i-1][j];
            }
        }

        for j in 1..n {
            for i in 0..n {
                diff[i][j] += diff[i][j-1];
            }
        }

        let mat: Vec<Vec<i32>> = diff[..n].iter().map(|row| row[..n].to_vec()).collect();
        mat
    }
}