impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let b = s.as_bytes();
        let len = b.len();
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let mut reachable_diff: Vec<i32> = vec![0; len+max_jump+1];
        if b[len-1] == b'1' {
            return false;
        }

        reachable_diff[0] = 1;
        reachable_diff[1] = -1;
        let mut sum = 0;

        for (idx, by) in b.iter().enumerate() {
            sum += reachable_diff[idx];
            if sum <= 0 || *by == b'1' {
                continue;
            }

            if idx == len - 1 {
                return true;
            }   

            reachable_diff[idx + min_jump] += 1;
            reachable_diff[idx + max_jump + 1] -= 1;
        } 

        sum > 0
    }
}