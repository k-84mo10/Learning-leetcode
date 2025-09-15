use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken: HashSet<char> = broken_letters.chars().collect();

        text.split_whitespace()
            .filter(|w| !w.chars().any(|c| broken.contains(&c)))
            .count() as i32
    }
}