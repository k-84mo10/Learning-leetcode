use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let length = s.len();
        if length <= 10 { return Vec::new(); }

        let mut freq: HashMap<&str, i32> = HashMap::new();
        let mut result: Vec<String> = Vec::new();

        for i in 0..=length-10 {
            let subslice = &s[i..i+10];
            let c = freq.entry(subslice).or_insert(0);
            if *c == 1 {
                result.push(subslice.to_string());
            } 
            *c += 1;
        }

        result
    }
}