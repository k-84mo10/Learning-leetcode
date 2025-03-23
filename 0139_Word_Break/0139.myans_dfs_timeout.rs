use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        Self::dfs(&s, 0, &word_set)
    }

    fn dfs (s: &String, index: usize, word_set: &HashSet<String>) -> bool {
        if index == s.len() {
            return true;
        }

        for i in index+1..=s.len() {
            let sub_s = &s[index..i];
            if word_set.contains(sub_s) {
                if Self::dfs(s, i, word_set) {
                    return true;
                }
            }
        }
        false
    }
}