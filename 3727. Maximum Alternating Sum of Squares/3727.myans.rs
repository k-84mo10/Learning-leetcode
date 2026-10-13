impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squares: Vec<i64> = nums
            .into_iter()
            .map(|x| (x as i64).pow(2))
            .collect();

        let k = squares.len() / 2;

        squares.select_nth_unstable(k);

        let minus: i64 = squares[..k].iter().sum();
        let plus: i64 = squares[k..].iter().sum();

        plus - minus
    }
}