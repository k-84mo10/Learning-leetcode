impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for i in 0..10 {
            let candidate = n + i;
            let product = Self::calc_product_of_digits(candidate);
            if product % t == 0 {
                return candidate;
            }
        }

        -1
    }

    fn calc_product_of_digits(n: i32) -> i32 {
        let mut res = 1;
        let mut n = n;

        while n != 0 {
            res *= n % 10;
            n /= 10;
        }

        res
    }
}