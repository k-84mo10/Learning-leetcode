impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower: u32 = 0;
        let mut upper: u32 = 0;
        let mut nospecial: u32 = 0;

        for b in word.bytes() {
            if b'a' <= b && b <= b'z' {
                let bit = 1 << (b - b'a');
                lower |= bit;
                nospecial |= upper & bit;
            } else if b'A' <= b && b <= b'Z' {
                upper |= 1 << (b - b'A');
            }
        }

        (lower & upper & !nospecial).count_ones() as i32
    }
}