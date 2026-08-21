impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn calc_height(first: i32, second: i32) -> i32 {
            // height is even 
            let k_even1 = (first as f64).sqrt() as i32;
            let k_even2 = (((second as f64 * 4.0 + 1.0).sqrt() - 1.0) / 2.0) as i32;

            let even = 2 * k_even1.min(k_even2);

            // height is odd
            let k_odd1 = k_even1 - 1;
            let k_odd2 = k_even2;

            let odd = if k_odd1 >= 0 {
                2 * k_odd1.min(k_odd2) + 1
            } else {
                0
            };

            even.max(odd)
        }

        calc_height(red, blue).max(calc_height(blue, red))
    }
}