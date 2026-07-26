impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2, mut max3) = (i32::MIN, i32::MIN, i32::MIN);
        let (mut min1, mut min2) = (i32::MAX, i32::MAX);
        
        for num in nums {
            if num > max1 {
                (max1, max2, max3) = (num, max1, max2);
            } else if num > max2 {
                (max2, max3) = (num, max2);
            } else if num > max3 {
                max3 = num;
            }

            if num < min1 {
                (min1, min2) = (num, min1); 
            } else if num < min2 {
                min2 = num;
            }
        }

        (max1 * max2 * max3).max(max1 * min1 * min2)
    }
}