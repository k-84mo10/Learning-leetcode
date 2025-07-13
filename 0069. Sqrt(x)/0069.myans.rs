impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 { return x; }

        let (mut left, mut right) = (1, x/2);
        while left <= right {
            let middle = (left + right) / 2;
            let square = middle as i64 * middle as i64;

            if square == x as i64{
                return middle;
            } else if square < x as i64{
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        right
    }
}