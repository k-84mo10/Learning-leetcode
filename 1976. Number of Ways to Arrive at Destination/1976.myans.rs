use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    fn build_graph (n: usize, roads: Vec<Vec<i32>>) -> Vec<Vec<(usize, i32)>> {
        let mut graph = vec![vec![]; n];

        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let cost = road[2];

            graph[u].push((v, cost));
            graph[v].push((u, cost));
        }

        graph
    }

    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;

        let graph = Self::build_graph(n, roads);

        let mut min_dist = vec![i64::MAX; n];
        let mut way = vec![0i64; n];

        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        min_dist[0] = 0;
        way[0] = 1;
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((curr_dist, u))) = heap.pop() {
            if curr_dist > min_dist[u] {
                continue;
            }

            for &(v, cost) in &graph[u] {
                let new_dist = curr_dist + cost as i64;
                if new_dist < min_dist[v] {
                    min_dist[v] = new_dist;
                    way[v] = way[u];
                    heap.push(Reverse((new_dist, v)));
                } else if new_dist == min_dist[v] {
                    way[v] = (way[v] + way[u]) % MOD;
                }
            }
        }
        way[n-1] as i32
    }
}