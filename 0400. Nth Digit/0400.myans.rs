impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut digit = 1_i64;
        let mut count = 9_i64;
        let mut start = 1_i64;

        while n > digit * count {
            n -= digit * count;
            start *= 10;
            count *= 10;
            digit += 1;
        } 

        let num = start + (n - 1) / digit;
        let idx = (n - 1) % digit;

        ((num / 10_i64.pow((digit - 1 - idx) as u32)) % 10) as i32
    }
}