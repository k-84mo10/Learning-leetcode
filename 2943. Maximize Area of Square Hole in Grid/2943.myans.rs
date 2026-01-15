impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let max_h = Self::max_consecutive_run(h_bars);
        let max_v = Self::max_consecutive_run(v_bars);
        
        let edge = max_h.min(max_v) + 1;

        edge*edge
    }

    fn max_consecutive_run(mut bars: Vec<i32>) -> i32 {
        bars.sort_unstable();

        let mut best: i32 = 1;
        let mut cur: i32 = 1;

        for w in bars.windows(2) {
            let prev = w[0];
            let next = w[1];

            if next == prev + 1 {
                cur += 1;
                if cur > best {
                    best = cur;
                }
            } else {
                cur = 1;
            }
        }

        best
    }
}