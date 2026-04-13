impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // 平方数の数が答え
        let n = n as i64;
        let (mut left, mut right) = (0i64, n);
        let mut ans = 0;

        while left <= right {
            let mid = (right - left) / 2 + left;
            if n >= mid * mid {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans as i32
    }
}