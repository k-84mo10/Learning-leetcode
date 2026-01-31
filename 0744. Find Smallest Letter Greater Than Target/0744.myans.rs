impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len()-1);
        if letters[right] <= target {
            return letters[left];
        }
        
        while left < right {
            let mid = left + (right - left) / 2;

            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        letters[left]
    }
}