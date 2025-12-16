impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();

        nums.sort_by(|a, b| {
            let ab = a.clone() + b;
            let ba = b.clone() + a;
            ba.cmp(&ab)
        });

        if nums[0] == "0" {
            return "0".to_string();
        }

        nums.concat()
    }
}