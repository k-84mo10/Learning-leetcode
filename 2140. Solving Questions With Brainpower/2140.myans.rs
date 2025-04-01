impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp: Vec<i64> = vec![0; n+1];

        for i in (0..n).rev() {
            let points = questions[i][0] as i64;
            let brainpower = questions[i][1] as usize;
            let next = i + brainpower + 1;
            dp[i] = dp[i+1].max(points + if (next < n) {dp[next]} else {0});
        }
        dp[0]
    }
}