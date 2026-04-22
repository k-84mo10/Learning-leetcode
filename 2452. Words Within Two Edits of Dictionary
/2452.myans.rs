impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut answer: Vec<String> = Vec::new();

        for query in &queries {
            for word in &dictionary {
                if Self::diff_count(&query, &word) <= 2 {
                    answer.push(query.to_string());
                    break;
                }
            }
        }

        answer
    }

    fn diff_count(a: &str, b: &str) -> usize {
        a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count()
    }
}