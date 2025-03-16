impl Solution {
    fn canRepairCarsInTime(ranks: &[i32], cars: i32, time: i64) -> bool {
        let mut sum = 0;
        for &rank in ranks.iter() {
            sum += ((time / rank as i64) as f64).sqrt() as i32;
            if sum >= cars {
                return true
            }
        }
        false
    }

    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        // rankの最小値を取る
        let min_rank = *ranks.iter().min().unwrap();

        let mut left = 0;
        let mut right = min_rank as i64 * cars as i64 * cars as i64;
        while left < right {
            let middle = left + (right - left) / 2;
            if Self::canRepairCarsInTime(&ranks, cars, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}