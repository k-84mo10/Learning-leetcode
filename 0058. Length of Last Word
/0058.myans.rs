impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed_s = s.trim_end();

        trimmed_s.chars().rev().take_while(|&c| c != ' ').count() as i32
    }
}