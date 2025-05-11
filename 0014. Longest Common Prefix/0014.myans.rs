impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common_prefix: Vec<char> = strs[0].chars().collect();

        for s in &strs[1..] {
            let current_chars: Vec<char> = s.chars().collect();
            let mut new_len = 0;

            for (a, b) in common_prefix.iter().zip(&current_chars) {
                if a != b { break; }
                new_len += 1;
            }

            common_prefix.truncate(new_len);
            if common_prefix.is_empty() { return "".to_string(); }
        }

        common_prefix.iter().collect()
    }
}