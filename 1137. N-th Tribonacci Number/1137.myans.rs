impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => return 0,
            1 | 2=> return 1,
            _ => {}
        }

        let (mut t0, mut t1, mut t2) = (0, 1, 1);

        for _ in 3..=n as usize{
            let next = t2 + t1 + t0;
            t0 = t1;
            t1 = t2;
            t2 = next;
        }

        t2
    }
}