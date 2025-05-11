impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for &x in &arr {
            if x % 2 == 1 {
                count += 1;
                if count == 3 { return true; }
            } else {
                count = 0;
            }
        }
        false
    }
}