use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut freq: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            match freq.entry(c) {
                Entry::Occupied(mut o) => {
                    let v = o.get_mut();
                    if *v == 0 {
                        return false;
                    }
                    *v -= 1;
                    if *v == 0 {
                        o.remove();
                    }
                }
                Entry::Vacant(_) => return false,
            }
        }

        freq.is_empty()
    }
}