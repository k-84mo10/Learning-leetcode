impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut answer = vec![-1; length];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..(2*length) {
            let idx = i % length;
            let cur = nums[idx];

            while let Some(&top) = stack.last() {
                if nums[top] < cur {
                    answer[top] = cur;
                    stack.pop();
                } else {
                    break;
                }
            }
            
            if i < length {
                stack.push(i);
            }
        }
        answer
    }
}