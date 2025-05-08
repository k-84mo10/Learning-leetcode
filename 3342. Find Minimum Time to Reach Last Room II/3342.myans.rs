use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        // ダイクストラ
        let n = move_time.len();
        let m = move_time[0].len();
        const inf : i32 = i32::MAX;

        let mut pq: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut min_time: Vec<Vec<i32>> = vec![vec![inf; m]; n];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

        pq.push(Reverse((0, 0, 0)));
        min_time[0][0] = 0;    

        while let Some(Reverse((time, x, y))) = pq.pop() {
            if visited[x][y] { continue; }
            visited[x][y] = true;

            if x == n-1 && y == m-1 { break; }
            
            let delay = if (x+y)%2 == 0 {1} else {2};

            for (nx, ny) in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
                if nx >= n || ny >= m { continue; }
                let new_time = move_time[nx][ny].max(time)+delay;
                if new_time < min_time[nx][ny] { 
                    min_time[nx][ny] = new_time;
                    pq.push(Reverse((new_time, nx, ny)));
                }
            }
        }

        min_time[n-1][m-1]
    }
}