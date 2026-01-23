impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n <= 2 {
            return 0;
        }

        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut p = 2usize;

        while p*p < n {
            if is_prime[p] {
                let mut m = p*p;
                while m < n {
                    is_prime[m] = false;
                    m += p;
                }
            }
            p += 1;
        }

        (2..n).filter(|&i| is_prime[i]).count() as i32
    }
}