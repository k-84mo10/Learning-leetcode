impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // xがマイナスならまずpalindromeではない
        if x < 0 {
            return false;
        }

        let num_s:Vec<char> = x.to_string().chars().collect();
        let (mut left, mut right) = (0, num_s.len()-1);

        while left < right {
            if num_s[left] != num_s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}