impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut even_count = [0i32; 26];
        let mut odd_count = [0i32; 26];

        for (i, (&b1, &b2)) in s1.as_bytes().iter().zip(s2.as_bytes()).enumerate() {
            let idx1 = (b1 - b'a') as usize;
            let idx2 = (b2 - b'a') as usize;

            if i % 2 == 0 {
                even_count[idx1] += 1;
                even_count[idx2] -= 1; 
            } else {
                odd_count[idx1] += 1;
                odd_count[idx2] -= 1;
            }
        }

        even_count.iter().all(|&x| x == 0) && odd_count.iter().all(|&x| x == 0)
    }
}