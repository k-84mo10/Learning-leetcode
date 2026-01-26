impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut minimum_diff = i32::MAX;
        for w in arr.windows(2) {
            let (a, b) = (w[0], w[1]);
            let diff = b - a;
            if diff < minimum_diff {
                answer = vec![vec![a, b]];
                minimum_diff = diff;
            } else if diff == minimum_diff {
                answer.push(vec![a, b]);
            }
        }

        answer
    }
}