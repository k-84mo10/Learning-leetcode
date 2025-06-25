impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut path = Vec::new();
        let mut result = Vec::new();
        Self::backtrack(0, target, &candidates, &mut path, &mut result);
        result
    }

    fn backtrack(
        start: usize, 
        target: i32, 
        candidates: &Vec<i32>, 
        path: &mut Vec<i32>, 
        result: &mut Vec<Vec<i32>>
    ) {
        if target == 0 {
            result.push(path.clone());
            return;
        }

        if target < 0 {
            return;
        }

        // 1文字目, 2文字目, 3文字目..と増やしていく
        for i in start..candidates.len() {
            // n文字目において、同じ文字が選択されないようにスキップ
            if i > start && candidates[i] == candidates[i-1] {
                continue;
            }

            path.push(candidates[i]);
            Self::backtrack(i+1, target-candidates[i], candidates, path, result);
            path.pop();
        }
    }
}