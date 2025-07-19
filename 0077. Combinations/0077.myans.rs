impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();
        let mut current = Vec::new();
        Self::dfs(n, k, 1, &mut current, &mut combinations);
        combinations
    }

    fn dfs(n: i32, k: i32, index: i32, current: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
        if current.len() == (k as usize) {
            combinations.push(current.clone());
            return;
        }

        for i in index..=n {
            current.push(i);
            Self::dfs(n, k, i+1, current, combinations);
            current.pop();
        }
    }
}