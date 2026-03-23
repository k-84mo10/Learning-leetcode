impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut rows = grid.into_iter();
        let first_row = rows.next().unwrap();

        let cols = first_row.len();
        let mut max_dp = vec![0_i64; cols];
        let mut min_dp = vec![0_i64; cols];

        // 1行目初期化
        let mut first_iter = first_row.into_iter();
        let first = first_iter.next().unwrap() as i64;
        max_dp[0] = first;
        min_dp[0] = first;

        let mut acc = first;
        for ((mx, mn), cell) in max_dp
            .iter_mut()
            .zip(min_dp.iter_mut())
            .skip(1)
            .zip(first_iter)
        {
            acc *= cell as i64;
            *mx = acc;
            *mn = acc;
        }

        for row in rows {
            let mut row_iter = row.into_iter();

            // 先頭列
            let first_cell = row_iter.next().unwrap() as i64;
            let a = max_dp[0] * first_cell;
            let b = min_dp[0] * first_cell;
            max_dp[0] = a.max(b);
            min_dp[0] = a.min(b);

            // 左側の最新値を保持
            let mut left_max = max_dp[0];
            let mut left_min = min_dp[0];

            for ((up_max, up_min), cell) in max_dp
                .iter_mut()
                .zip(min_dp.iter_mut())
                .skip(1)
                .zip(row_iter)
            {
                let val = cell as i64;

                let candidates = [
                    *up_max * val,
                    *up_min * val,
                    left_max * val,
                    left_min * val,
                ];

                let new_max = *candidates.iter().max().unwrap();
                let new_min = *candidates.iter().min().unwrap();

                *up_max = new_max;
                *up_min = new_min;

                left_max = new_max;
                left_min = new_min;
            }
        }

        let ans = max_dp.into_iter().last().unwrap();
        if ans < 0 {
            -1 
        } else {
            (ans % MOD) as i32
        }
    }
}