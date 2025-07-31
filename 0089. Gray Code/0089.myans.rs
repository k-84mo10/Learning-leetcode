impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let size = 1 << n;
        (0..size).map(|i| i ^ (i >> 1)).collect()
    }
}