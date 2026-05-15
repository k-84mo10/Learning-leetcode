impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let (mut left, mut right) = (1, num);

        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;
            if num == square {
                return true;
            } else if num > square {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}