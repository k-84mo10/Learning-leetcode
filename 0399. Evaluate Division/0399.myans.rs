use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        for (equation, &val) in equations.iter().zip(values.iter()) {
            let numerator = equation[0].clone();
            let denominator = equation[1].clone();

            graph
                .entry(numerator.clone())
                .or_insert_with(Vec::new)
                .push((denominator.clone(), val));
            graph
                .entry(denominator.clone())
                .or_insert_with(Vec::new)
                .push((numerator.clone(), 1.0 / val));
        }

        let mut answers = Vec::with_capacity(queries.len());

        for q in queries.iter() {
            let numerator = &q[0];
            let denominator = &q[1];

            if !graph.contains_key(numerator) || !graph.contains_key(denominator) {
                answers.push(-1.0);
                continue;
            }

            if numerator == denominator {
                answers.push(1.0);
                continue;
            }

            let mut visited = HashSet::new();
            let result = Self::dfs(numerator, denominator, 1.0, &mut visited, &graph);
            answers.push(result.unwrap_or(-1.0));
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
                    if let Some(result) =
                        Self::dfs(next, target, acc * *weight, visited, graph)
                    {
                        return Some(result);
                    }
                }
            }
        }

        None
    }
}
