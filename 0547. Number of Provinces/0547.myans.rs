impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut is_detected: Vec<bool> = vec![false; n];

        let mut province = 0;
        for i in 0..n as usize {
            if !is_detected[i] {
                province += 1;
                is_detected[i] = true;
                Self::dfs(i, n, &is_connected, &mut is_detected);
            }
        }

        province
    }

    fn dfs(index: usize, n: usize, is_connected: &Vec<Vec<i32>>, is_detected: &mut Vec<bool>) {
        for i in 0..n {
            if is_connected[index][i] == 1 && !is_detected[i] {
                is_detected[i] = true;
                Self::dfs(i, n, is_connected, is_detected);
            }
        }
    }
}