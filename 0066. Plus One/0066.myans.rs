impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new_digits = Vec::new();
        let mut carry = 1;

        for i in digits.iter().rev() {
            let new_digit = i + carry;
            carry = new_digit / 10;
            new_digits.push(new_digit % 10);
        }

        if carry != 0 { new_digits.push(carry); }

        new_digits.reverse();
        new_digits
    }
}