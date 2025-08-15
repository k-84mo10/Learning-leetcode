impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n < 0 {return false;}
        let n = n as u32;
        n.is_power_of_two() && (n & 0x5555_5555) != 0
    }
}