impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let mut sum = 0;
        let mut q = 0;

        for (idx, c) in num.chars().enumerate() {
            let sign = if idx < n/2 { 1 } else { -1 };

            if c == '?' {
                q += sign;
            } else {
                sum += sign * c.to_digit(10).unwrap() as i32;
            }
        }

        2 * sum != -9 * q
    }
}