impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let len = nums.len();
        let full = (1usize << len) - 1;

        let mut can_special: Vec<Vec<bool>> = vec![vec![false; len]; len];

        for i in 0..len {
            for j in i+1..len {
                if nums[i] % nums[j] == 0 || nums[j] % nums[i] == 0 {
                    can_special[i][j] = true;
                    can_special[j][i] = true;
                }
            }
        }

        let mut memo = vec![vec![-1i64; len]; 1usize << len];

        fn dfs (
            mask: usize,
            last: usize,
            can_special: &Vec<Vec<bool>>,
            memo: &mut Vec<Vec<i64>>,
            full: usize,
        ) -> i64 {
            const MOD: i64 = 1_000_000_007;

            if mask == full {
                return 1;
            }

            if memo[mask][last] != -1 {
                return memo[mask][last];
            }

            let len = can_special.len();
            let mut result = 0i64;

            for nxt in 0..len {
                if mask & (1usize << nxt) != 0 {
                    continue;
                }

                if can_special[last][nxt] {
                    result += dfs(mask|(1usize << nxt), nxt, can_special, memo, full);
                    result %= MOD;
                }
            }

            memo[mask][last] = result;
            result
        }

        let mut ans = 0i64;

        for i in 0..len {
            ans += dfs(1usize << i, i, &can_special, &mut memo, full);
            ans %= MOD;
        }

        ans as i32
    }
}