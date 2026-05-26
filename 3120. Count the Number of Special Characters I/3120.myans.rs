impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower: u32 = 0;
        let mut upper: u32 = 0;

        for b in word.bytes() {
            if b'a' <= b && b <= b'z' {
                lower |= 1 << (b - b'a');
            } else if b'A' <= b && b <= b'Z' {
                upper |= 1 << (b - b'A');
            }
        }

        (lower & upper).count_ones() as i32
    }
}