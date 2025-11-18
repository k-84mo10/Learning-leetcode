impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = Vec::new();
        let mut result: Vec<Vec<i32>> = Vec::new();

        Self::dfs(1, k, n, &mut path, &mut result);

        result
    }

    fn dfs (
        start: i32,
        k_left: i32,
        remain: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>
    ) {
        if k_left == 0 {
            if remain == 0 {
                result.push(path.clone());
            }
            return;
        }

        for num in start..=9 {
            if num > remain {
                break;
            }

            path.push(num);
            Self::dfs(num+1, k_left-1, remain-num, path, result);
            path.pop();
        }
    }
}