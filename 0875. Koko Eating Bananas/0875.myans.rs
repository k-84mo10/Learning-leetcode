impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::eat_banana(&piles, mid, h) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn eat_banana(piles: &Vec<i32>, k: i32, h: i32) -> bool {
        let mut time = 0;
        for pile in piles {
            time += (pile-1) / k + 1;
        }
        h >= time
    }
}