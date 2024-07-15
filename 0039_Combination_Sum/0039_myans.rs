impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut route: Vec<i32> = Vec::new();
        Self::dfs(&candidates, target, 0, &mut route, &mut answer);
        answer
    }

    fn dfs(candidates: &Vec<i32>, target: i32, start: usize, route: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if target == 0 {
            answer.push(route.clone());
            return;
        }

        for i in start..candidates.len() {
            if target >= candidates[i] {
                route.push(candidates[i]);
                Self::dfs(&candidates, target - candidates[i], i, route, answer);
                route.pop();
            }
        }
    }
}