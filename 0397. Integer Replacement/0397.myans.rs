impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as u32;
        let mut count = 0;

        while n > 1 {
            let zeros = n.trailing_zeros();

            if zeros > 0 {
                count += zeros as i32;
                n >>= zeros;
                continue;
            }

            // n が 奇数
            // n = 3 だけ 3->2->1 の方が速い
            n = if n == 3 {
                2
            } else if (n+1).trailing_zeros() > (n-1).trailing_zeros() {
                n + 1
            } else {
                n - 1
            };

            count += 1;
        }

        count
    }
}