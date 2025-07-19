impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut red, mut white, mut blue) = (0, 0, 0);

        for i in 0..nums.len() {
            if nums[i] == 0 { red += 1; }
            if nums[i] == 1 { white += 1; }
            if nums[i] == 2 { blue += 1; }
        }

        for i in 0..red as usize {
            nums[i] = 0;
        }
        for i in red as usize..(red + white) as usize {
            nums[i] = 1;
        } 
        for i in (red + white) as usize .. (red + white + blue) as usize {
            nums[i] = 2;
        }
    }
}