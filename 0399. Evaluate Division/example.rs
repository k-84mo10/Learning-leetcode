use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // グラフ: 変数 -> (隣接変数, 比率)
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // グラフ構築
        for (eq, &val) in equations.iter().zip(values.iter()) {
            let a = &eq[0];
            let b = &eq[1];

            graph
                .entry(a.clone())
                .or_insert_with(Vec::new)
                .push((b.clone(), val));
            graph
                .entry(b.clone())
                .or_insert_with(Vec::new)
                .push((a.clone(), 1.0 / val));
        }

        // 各クエリを処理
        let mut answers = Vec::with_capacity(queries.len());

        for q in queries {
            let c = &q[0];
            let d = &q[1];

            // どちらかの変数がグラフに存在しない場合
            if !graph.contains_key(c) || !graph.contains_key(d) {
                answers.push(-1.0);
                continue;
            }

            // C / C なら 1.0
            if c == d {
                answers.push(1.0);
                continue;
            }

            let mut visited = HashSet::new();
            let res = Self::dfs(c, d, 1.0, &mut visited, &graph);
            answers.push(res.unwrap_or(-1.0));
        }

        answers
    }

    fn dfs(
        current: &str,
        target: &str,
        acc: f64,
        visited: &mut HashSet<String>,
        graph: &HashMap<String, Vec<(String, f64)>>,
    ) -> Option<f64> {
        if current == target {
            return Some(acc);
        }

        visited.insert(current.to_string());

        if let Some(neighbors) = graph.get(current) {
            for (next, weight) in neighbors {
                if !visited.contains(next) {
                    if let Some(result) = Self::dfs(next, target, acc * *weight, visited, graph) {
                        return Some(result);
                    }
                }
            }
        }

        None
    }
}
