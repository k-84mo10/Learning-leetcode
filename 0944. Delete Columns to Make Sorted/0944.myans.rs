impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let bytes: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

        let rows = bytes.len();
        let cols = bytes[0].len();

        let mut answer = 0;

        for col in 0..cols {
            for row in 1..rows {
                if bytes[row-1][col] > bytes[row][col] {
                    answer += 1;
                    break;
                }
            }
        }

        answer
    }
}