impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut nums = nums;
        Self::dfs_swapper(&mut nums, &mut answer, 0);
        answer
    }

    fn dfs_swapper(nums: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>, index: usize) {
        if index == nums.len() {
            answer.push(nums.clone());
            return;
        } 

        for i in index..nums.len() {
            nums.swap(i, index);
            Self::dfs_swapper(nums, answer, index+1);
            nums.swap(i, index);
        }
    }
}