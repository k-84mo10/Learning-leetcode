impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut diff = vec![0; (2*limit+2) as usize];

        for i in 0..(n+1)/2 {
            let a = nums[i].min(nums[n-1-i]);
            let b = nums[i].max(nums[n-1-i]);

            diff[2] += 2;
            diff[(a+1) as usize] -= 1;
            diff[(a+b) as usize] -= 1;
            diff[(a+b+1) as usize] += 1;
            diff[(b+limit+1) as usize] += 1; 
        }

        let mut current = 0;
        let mut min_ops = i32::MAX;

        for i in 2..(2*limit+1) as usize {
            current += diff[i];
            min_ops = min_ops.min(current);
        }

        min_ops
    }
}