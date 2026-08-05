use std::collections::VecDeque;

impl Solution {
    pub fn remaining_methods(
        n: i32,
        k: i32,
        invocations: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        let graph = Self::create_graph(n, &invocations);
        let suspicious = Self::find_suspicious_methods(k, &graph);

        // 疑わしくないメソッドから疑わしいメソッドへの呼び出しがあるか確認
        for invocation in &invocations {
            let from = invocation[0] as usize;
            let to = invocation[1] as usize;

            if !suspicious[from] && suspicious[to] {
                return (0..n as i32).collect();
            }
        }

        (0..n)
            .filter(|&method| !suspicious[method])
            .map(|method| method as i32)
            .collect()
    }

    fn create_graph(
        n: usize,
        invocations: &[Vec<i32>],
    ) -> Vec<Vec<usize>> {
        let mut graph = vec![Vec::new(); n];

        for invocation in invocations {
            let from = invocation[0] as usize;
            let to = invocation[1] as usize;

            graph[from].push(to);
        }

        graph
    }

    fn find_suspicious_methods(
        k: usize,
        graph: &[Vec<usize>],
    ) -> Vec<bool> {
        let mut suspicious = vec![false; graph.len()];
        let mut queue = VecDeque::new();

        suspicious[k] = true;
        queue.push_back(k);

        while let Some(node) = queue.pop_front() {
            for &next in &graph[node] {
                if !suspicious[next] {
                    suspicious[next] = true;
                    queue.push_back(next);
                }
            }
        }

        suspicious
    }
}