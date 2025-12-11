impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let papers = citations.len();
        let mut citations = citations;
        citations.sort_by(|a, b| b.cmp(a));

        for i in 0..papers {
            let h = (i + 1) as i32;
            if citations[i] < h {
                return i as i32;
            }
        }

        papers as i32
    }
}