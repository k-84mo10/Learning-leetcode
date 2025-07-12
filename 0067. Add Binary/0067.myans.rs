impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_chars: Vec<char> = a.chars().rev().collect();
        let b_chars: Vec<char> = b.chars().rev().collect();

        let mut answer: Vec<char> = Vec::new();
        let mut digit = 0;
        let mut carry = 0;

        while a_chars.len() > digit || b_chars.len() > digit || carry != 0 {
            let a_m = a_chars.get(digit).and_then(|c| c.to_digit(2)).unwrap_or(0);
            let b_m = b_chars.get(digit).and_then(|c| c.to_digit(2)).unwrap_or(0);
            let sum = a_m + b_m + carry;
            carry = sum / 2;
            answer.push(char::from_digit(sum%2, 2).unwrap_or('0'));
            digit += 1;
        }

        answer.iter().rev().collect::<String>()
    }
}
