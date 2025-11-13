impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut count_1 = 0;
        let mut result = 0;

        let mut prev = b'0';

        for b in s.bytes() {
            match b {
                b'1' => {
                    count_1 += 1;
                }
                b'0' => {
                    if prev == b'1' {
                        result += count_1;
                    }
                }
                _ => {}
            }
            prev = b;
        }

        result
    }
}