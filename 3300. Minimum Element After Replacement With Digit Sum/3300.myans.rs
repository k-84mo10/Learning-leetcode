impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut min_element = i32::MAX;

        for num in nums {
            min_element = min_element.min(Self::calc_sum_of_digits(num));
            if min_element == 1 {
                return 1;
            }
        }

        min_element
    }

    fn calc_sum_of_digits(num: i32) -> i32 {
        let mut sum = 0;
        let mut num = num;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}