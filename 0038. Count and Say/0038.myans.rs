impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let pastString = Self::count_and_say(n-1);

        Self::show_RLE(pastString)
    }

    fn show_RLE(s: String) -> String {
        let mut rle = String::new();
        let mut chars = s.chars().peekable();

        while let Some(current_char) = chars.next() {
            let mut count = 1;

            while let Some(&next_char) = chars.peek() {
                if next_char == current_char {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }

            rle.push_str(&count.to_string());
            rle.push(current_char);
        }

        rle
    }
}