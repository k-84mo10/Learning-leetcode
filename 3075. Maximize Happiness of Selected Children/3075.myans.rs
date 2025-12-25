impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let happiness = happiness;
        happiness.sort_by(|a, b| b.cmp(a));

        let turns = k as usize;
        let mut total_happiness = 0;

        for turn in 0..turns {
            let current_happiness = happiness[i] as i64 - turn as i64
        }
    }
}