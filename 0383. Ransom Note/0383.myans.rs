use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letter_counts: HashMap<char, usize> = HashMap::new();

        for c in magazine.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            match letter_counts.get_mut(&c) {
                Some(counts) if *counts > 0 => *counts -= 1,
                _ => return false,
            }
        }

        true
    }
}