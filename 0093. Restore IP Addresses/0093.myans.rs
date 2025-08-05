impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut path = Vec::new();
        let mut result = Vec::new();

        Self::dfs(&chars, 0, 0, &mut path, &mut result);

        result
    }

    fn dfs(chars: &[char], index: usize, segment_num: usize, path: &mut Vec<String>, result: &mut Vec<String>) {
        if segment_num == 4 {
            if index == chars.len() {
                result.push(path.join("."));
            }
            return;
        }

        for length in 1..=3 {
            if index + length > chars.len() {
                break;
            }
            let segment: String = chars[index..index + length].iter().collect();

            // 先頭0禁止
            if segment.starts_with('0') && segment.len() > 1 {
                continue;
            }

            // 数値範囲チェック
            if let Ok(num) = segment.parse::<u8>() {
                if num <= 255 {
                    path.push(segment);
                    Self::dfs(chars, index+length, segment_num + 1, path, result);
                    path.pop();
                }
            }
        }
    }
}