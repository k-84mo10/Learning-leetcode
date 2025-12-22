use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut p2w: HashMap<char, &str> = HashMap::new();
        let mut w2p: HashMap<&str, char> = HashMap::new();

        let mut p_it = pattern.chars();
        let mut s_it = s.split_whitespace();

        loop {
            match (p_it.next(), s_it.next()) {
                (None, None) => return true,
                (Some(_), None) | (None, Some(_)) => return false,
                (Some(p), Some(w)) => {
                    match p2w.entry(p) {
                        Entry::Occupied(e) => {
                            if *e.get() != w { return false; }
                        }
                        Entry::Vacant(e) => {
                            if w2p.contains_key(w) { return false; }
                            e.insert(w);
                            w2p.insert(w, p);
                        }
                    }
                }
            } 
        }
    }
}