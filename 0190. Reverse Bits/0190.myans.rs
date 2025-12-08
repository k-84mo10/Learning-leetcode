impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;

        for _ in 0..32 {
            res = (res << 1) | (n & 1);
            n >>= 1;
        }

        res
    }
}