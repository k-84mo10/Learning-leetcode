impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bit_sum: Vec<i32> = vec![0; (n+1) as usize];
        let mut largest_digit = 0;

        for i in 1..=n as usize {
            if Self::is_power_of_two(i as i32) {
                bit_sum[i] = 1;
                largest_digit = i;
            } else {
                bit_sum[i] = bit_sum[i-largest_digit] + 1;
            }
        }

        bit_sum
    }

    fn is_power_of_two(n: i32) -> bool {
        n != 0 && (n & (n-1) == 0)
    }
}