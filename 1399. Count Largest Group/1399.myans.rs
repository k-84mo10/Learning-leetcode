use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut largest_size = 0;
        let mut num_of_largest = 0;

        for i in 1..=n {
            let mut num = i;
            let mut sum = 0;
            while num != 0 {
                sum += num % 10;
                num /= 10;
            }
            let count = map.entry(sum).or_insert(0);
            *count += 1;

            if *count > largest_size {
                largest_size = *count;
                num_of_largest = 1;
            } else if *count == largest_size {
                num_of_largest += 1;
            }
        }
        num_of_largest
    }
}