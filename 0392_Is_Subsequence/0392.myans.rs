impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();

        let mut target = match s_iter.next() {
            Some(c) => c,
            None => return true,
        };

        for c in t.chars() {
            if c == target {
                match s_iter.next() {
                    Some(next_c) => target = next_c,
                    None => return true,
                }
            }
        }

        false
    }
}