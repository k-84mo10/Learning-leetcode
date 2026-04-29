impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut hidden = 0i64;

        let (mut cur_max, mut cur_min) = (0i64, 0i64);

        for diff in differences {
            hidden += diff as i64;
            cur_max = cur_max.max(hidden);
            cur_min = cur_min.min(hidden);
        }

        let possible = (upper - lower) as i64 - (cur_max - cur_min) + 1i64;
        if possible > 0 { possible as i32 } else { 0 }
    }
}