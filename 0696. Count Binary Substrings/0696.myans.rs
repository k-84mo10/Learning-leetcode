impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut cur_run = 1;
        let mut prev_run = 0;
        let mut ans = 0;

        for w in bytes.windows(2) {
            if w[0] == w[1] {
                cur_run += 1;
            } else {
                ans += cur_run.min(prev_run);
                prev_run = cur_run;
                cur_run = 1;
            }
        }

        ans + cur_run.min(prev_run)
    }
}