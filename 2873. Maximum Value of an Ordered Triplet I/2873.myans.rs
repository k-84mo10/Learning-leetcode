impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut current_max_value = 0;
        let mut current_big_distance = 0;
        let mut max_triplet_value = 0i64;

        for num in nums {
            max_triplet_value = max_triplet_value.max(current_big_distance*num as i64);
            current_max_value = current_max_value.max(num);
            current_big_distance = current_big_distance.max((current_max_value - num) as i64);
        }
        max_triplet_value
    }
}