impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        if s.chars().any(|c| vowels.contains(&c.to_ascii_lowercase())) {
            true
        } else {
            false
        }
    }
}