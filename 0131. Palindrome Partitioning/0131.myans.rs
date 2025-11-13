impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = s.chars().collect();
        let chars_size = chars.len();

        let mut is_pal: Vec<Vec<bool>> = vec![vec![false; chars_size]; chars_size];

        for i in 0..chars_size {
            is_pal[i][i] = true;
        }

        for len in 2..=chars_size {
            for start in 0..=chars_size - len {
                let end = start + len - 1;
                if chars[start] == chars[end] {
                    if len == 2 {
                        is_pal[start][end] = true;
                    } else {
                        is_pal[start][end] = is_pal[start+1][end-1];
                    }
                }
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        Self::dfs(0, &chars, &is_pal, &mut path, &mut result);
        result
    }

    fn dfs (
        start: usize,
        chars: &[char],
        is_pal: &[Vec<bool>],
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>
    ) {
        let chars_size = chars.len();

        if start == chars_size {
            result.push(path.clone());
            return;
        }

        for end in start..chars_size {
            if is_pal[start][end] {
                let substr: String = chars[start..=end].iter().collect();
                path.push(substr);
                Self::dfs(end+1, chars, is_pal, path, result);
                path.pop();
            }
        }
    }
}