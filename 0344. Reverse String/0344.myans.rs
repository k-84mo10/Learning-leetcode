impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut left, mut right) = (0, s.len()-1);

        while left < right {
            let tmp = s[left];
            s[left] = s[right];
            s[right] = tmp;
            left += 1;
            right -= 1; 
        } 
    }
}