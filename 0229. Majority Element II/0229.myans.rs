impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut candidate1, mut candidate2) = (0, 0);
        let (mut count1, mut count2) = (0, 0);

        for &x in &nums {
            if x == candidate1 {
                count1 += 1;
            } else if x == candidate2 {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = x;
                count1 += 1;
            } else if count2 == 0 {
                candidate2 = x;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        let mut result = Vec::new();
        let threshold = nums.len() / 3;
        count1 = nums.iter().filter(|&& x| x == candidate1).count();
        if count1 > threshold { result.push(candidate1) };
        if candidate1 == candidate2 { return result; }
        count2 = nums.iter().filter(|&& x| x == candidate2).count();
        if count2 > threshold { result.push(candidate2) };

        result
    }
}