impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (n - Self::reverse(n)).abs()
    }

    fn reverse(n: i32) -> i32 {
        let mut ans = 0;
        let mut n = n;

        while n > 0 {
            let cur = n % 10;
            n /= 10;
            ans *= 10;
            ans += cur;
        }

        ans
    }
}