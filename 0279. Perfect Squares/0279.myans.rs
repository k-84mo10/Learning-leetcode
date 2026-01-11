impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // ラグランジュの四平法定理より、答えは1～4に限定
        let root = (n as f32).sqrt() as i32;

        // n が平方数なら、答えは1
        if root*root == n {
            return 1;
        }

        // n が平方数の和で表せるなら、答えは2
        let half = n / 2;
        for num in 1..=root {
            let square = num*num;
            if square > half {
                break;
            }

            let difference = n - square;
            if Self::is_sqrt(difference) {
                return 2;
            }
        }

        // ルジャンドルの三平方の定理より、8n-7型なら答えは4
        let mut a = n;
        while a % 4 == 0 {
            a /= 4;
        }
        if a % 8 == 7 {
            return 4;
        }

        3
    }

    fn is_sqrt(n: i32) -> bool {
        let root = (n as f32).sqrt() as i32;
        root*root == n
    }
}