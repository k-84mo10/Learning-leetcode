impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let phone_combination = [
            "",
            "",
            "abc",
            "def",
            "ghi",
            "jkl",
            "mno",
            "pqrs",
            "tuv",
            "wxyz"
        ];

        let mut letter_combination: Vec<String> = Vec::new();
        if digits == "" {
            return letter_combination;
        }
        letter_combination.push("".to_string());

        for digit in digits.chars() {
            let digit = digit.to_digit(10).unwrap() as usize;
            let mut tmp = Vec::new();
            for letter in letter_combination {
                for w in phone_combination[digit].chars() {
                    tmp.push(letter.clone()+&w.to_string());
                }
            }
            letter_combination = tmp;
        }

        letter_combination
    }
}