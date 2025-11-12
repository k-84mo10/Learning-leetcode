impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let nums_size = nums.len();

        if Self::gcd_of_list(&nums) != 1 {
            return -1;
        }

        let count_ones = nums.iter().copied().filter(|&x| x == 1).count();
        if count_ones != 0 {
            return (nums_size - count_ones) as i32;
        }

        let mut minimum_length = nums_size;

        for i in 0..nums_size {
            let mut g = nums[i];
            for j in i+1..nums_size {
                g = Self::gcd(g, nums[j]);
                if g == 1 {
                    minimum_length = minimum_length.min(j - i + 1);
                    break;
                }
            }
        }

        (minimum_length + nums_size - 2) as i32
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a.abs()
    }

    fn gcd_of_list(nums: &[i32]) -> i32 {
        nums.iter().copied().reduce(|a, b| Self::gcd(a, b)).unwrap()
    }
}
