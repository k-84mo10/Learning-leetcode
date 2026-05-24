impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let doubled = s.clone() + &s;
        doubled[1..doubled.len()-1].contains(&s)
    }
}