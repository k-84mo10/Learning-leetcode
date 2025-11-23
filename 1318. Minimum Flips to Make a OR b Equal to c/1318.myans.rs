impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let x = !(a | b) & c;   // c=1 なのに、a|b = 0
        let y = !c & a;         // c=0 なのに、a=1 
        let z = !c & b;         // c=0 なのに、b=1

        (x.count_ones() + y.count_ones() + z.count_ones()) as i32
    }
}