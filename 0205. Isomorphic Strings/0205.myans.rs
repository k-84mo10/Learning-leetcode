use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s2t: HashMap<char, char> = HashMap::new();
        let mut map_t2s: HashMap<char, char> = HashMap::new();

        for (sc, tc) in s.chars().zip(t.chars()) {
            match map_s2t.entry(sc) {
                Entry::Vacant(e) => { e.insert(tc); }
                Entry::Occupied(e) => {
                    if *e.get() != tc { return false; }
                }
            }

            match map_t2s.entry(tc) {
                Entry::Vacant(e) => { e.insert(sc); }
                Entry::Occupied(e) => {
                    if *e.get() != sc { return false; }
                }
            }
        }

        true
    }
}