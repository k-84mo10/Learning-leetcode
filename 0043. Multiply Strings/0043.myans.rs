impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // どちらかが0なら、答えは0
        if num1 == "0" || num2 == "0" { return "0".to_string(); }

        let num1_i32vec = Self::string_to_i32_vec(&num1);
        let num2_i32vec = Self::string_to_i32_vec(&num2);

        let num1_len = num1_i32vec.len();
        let num2_len = num2_i32vec.len();

        let mut answer = vec![0; num1_len+num2_len];

        for i in 0..num1_len {
            for j in 0..num2_len {
                let a = num1_i32vec[num1_len-i-1];
                let b = num2_i32vec[num2_len-j-1];
                let product = a * b;

                let pos = i + j;
                answer[pos] += product;
                if answer[pos] >= 10 {
                    answer[pos+1] += answer[pos] / 10;
                    answer[pos] %= 10;
                }
            }
        }

        Self::i32vec_to_string(answer)
    }

    fn string_to_i32_vec(s: &str) -> Vec<i32> {
        s.chars().filter_map(|c| c.to_digit(10)).map(|d| d as i32).collect()
    }

    fn i32vec_to_string(mut v: Vec<i32>) -> String {
        // 末尾の0は削除
        while v.len() > 1 && *v.last().unwrap() == 0 {
            v.pop();
        }

        v.into_iter().rev().map(|d| std::char::from_digit(d as u32, 10).unwrap()).collect()
    }
}