impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut answer = vec![nums[0]];
        
        for &num in nums.iter().skip(1) {
            if &num > answer.last().unwrap() {
                answer.push(num);
            } else {
                match answer.binary_search_by(|probe| probe.cmp(&num)) {
                    Ok(_) => {},
                    Err(index) => answer[index] = num,
                }
            }
        }

        answer.len() as i32
    }
}