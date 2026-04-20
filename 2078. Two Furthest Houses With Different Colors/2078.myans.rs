impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        if colors.first() != colors.last() {
            return colors.len() as i32 - 1;
        }

        let n = colors.len();
        let edge_color = colors[0];
        let (mut left, mut right) = (0, n-1); 

        while left <= right {
            if colors[left] != edge_color {
                return (n - 1 - left) as i32;
            }

            if colors[right] != edge_color {
                return right as i32;
            }

            left += 1;
            right -= 1;
        }


        -1
    }
}