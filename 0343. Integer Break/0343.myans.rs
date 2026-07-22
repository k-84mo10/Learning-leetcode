impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // 特殊ケース（n <= 3) 
        if n <= 3 {
            return n - 1;
        }

        // 3 で割った余りによって分岐
        match n % 3 {
            0 => 3i32.pow((n/3) as u32),
            1 => 3i32.pow(((n/3) - 1) as u32) * 4,
            2 => 3i32.pow((n/3) as u32) * 2,
            _ => unreachable!()
        }
    }
}