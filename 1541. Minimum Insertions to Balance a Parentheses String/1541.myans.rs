impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut res = 0;
        let mut balance = 0;

        for c in s.chars() {
            if c == '(' {
                if balance % 2 == 1 {
                    res += 1;
                    balance -= 1;
                }
                balance += 2;
            } else {
                balance -= 1;
                if balance < 0 {
                    res += 1;
                    balance = 1;
                }
            }
        }

        res + balance
    }
}