impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut common = vec![i32::MAX; 26];

        for word in words {
            let mut current = vec![0; 26];

            for ch in word.chars() {
                current[ch as usize - 'a' as usize] += 1;
            }

            for i in 0..26 {
                common[i] = common[i].min(current[i]);
            }
        }

        let mut ans = Vec::new();

        for i in 0..26 {
            for _ in 0..(common[i] as usize) {
                ans.push(((b'a' + i as u8) as char).to_string());
            }
        }

        ans
    }
}