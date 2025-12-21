impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let bytes: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

        let rows = bytes.len();
        let cols = bytes[0].len();

        let mut determined: Vec<bool> = vec![false; rows];   // strs[row-1] < strs[row] が決まれば true
        let mut determined_count = 0;

        let mut answer = 0;

        for col in 0..cols {
            let mut is_delete = false;

            for row in 1..rows {
                if determined[row] { continue; }

                if bytes[row-1][col] > bytes[row][col] {
                    is_delete = true;
                    break;
                } 
            }

            if is_delete {
                answer += 1;
                continue;
            }

            for row in 1..rows {
                if determined[row] { continue; }

                if bytes[row-1][col] < bytes[row][col] {
                    determined[row] = true;
                    determined_count += 1;
                }
            }

            if determined_count == rows - 1 { break; }
        }

        answer
    }
}