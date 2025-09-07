impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut answer = Vec::new();

        let m = n / 2;

        for i in 1..=m {
            answer.push(i);
            answer.push(-i);
        }

        if n % 2 == 1 { answer.push(0); }

        answer
    }
}