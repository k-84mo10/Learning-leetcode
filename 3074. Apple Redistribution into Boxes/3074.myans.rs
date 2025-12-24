impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        capacity.sort_by(|a, b| b.cmp(a));

        let total_apples: i32 = apple.iter().sum();

        let mut acc = 0;
        for (idx, cap) in capacity.into_iter().enumerate() {
            acc += cap;
            if acc >= total_apples {
                return (idx + 1) as i32;
            }
        }

        -1
    }
}