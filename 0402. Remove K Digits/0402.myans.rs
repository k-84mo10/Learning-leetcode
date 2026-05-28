impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut k = k;

        for digit in num.chars() {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > digit {
                stack.pop();
                k -= 1;
            }
            stack.push(digit);
        }

        while k > 0 {
            stack.pop();
            k -= 1;
        }

        let ans: String = stack
            .into_iter()
            .skip_while(|&c| c == '0')
            .collect();

        if ans.is_empty() {
            "0".to_string()
        } else {
            ans
        }
    }
}