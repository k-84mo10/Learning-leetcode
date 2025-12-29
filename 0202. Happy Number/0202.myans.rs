use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut base = n;
        let mut seen: HashSet<i32> = HashSet::new();

        loop {
            if base == 1 { return true; }
            if !seen.insert(base) {
                return false;
            } 
            base = Self::calc_square_sum(base);
        }
    }

    fn calc_square_sum(n: i32) -> i32 {
        let mut calc = 0;
        let mut base = n;
        while base != 0 {
            let num = base % 10;
            calc += num * num;
            base /= 10;
        }
        calc
    }
}