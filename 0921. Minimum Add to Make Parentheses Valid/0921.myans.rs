impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut res = 0;
        let mut balance = 0;

        for c in s.chars() {
            if c == '(' {
                balance += 1;
            } else {
                balance -= 1;
                if balance < 0 {
                    res -= balance;
                    balance = 0;
                }
            }
        }

        res += balance;
        res
    }
}