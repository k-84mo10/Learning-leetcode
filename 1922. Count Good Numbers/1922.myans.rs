impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let modulo: i64 = 1_000_000_007;

        let even_indice = (n+1)/2;
        let odd_indice = n/2;

        (Self::mod_pow(5, even_indice, modulo) * Self::mod_pow(4, odd_indice, modulo) % modulo) as i32
    }

    fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
        let mut result = 1;
        base %= modulo;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result*base) % modulo;
            }
            base = (base * base) % modulo;
            exp /= 2;
        }
        result
    }
}