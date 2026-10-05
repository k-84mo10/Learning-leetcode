impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];

        for c in s.chars() {
            if c == '(' {
                stack.push(0);
                continue;
            }

            let inner = stack.pop().unwrap();
            let score = if inner == 0 {
                1
            } else {
                2 * inner
            };

            *stack.last_mut().unwrap() += score;
        }

        stack[0]
    }
}