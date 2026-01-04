impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let max_n = nums.iter().map(|&x| x as usize).max().unwrap_or(0);
        if max_n < 6 {
            return 0;
        }

        let spf = Self::create_spf(max_n);
        let mut sum = 0;

        for num in nums {
            let n = num as usize;
            let p = spf[n];
            if p == n {
                continue;
            }

            let q = n / p;

            if q == p*p {
                sum += (1 + p + q + n) as i32 ;
                continue
            }

            if n % (p * p) != 0 && spf[q] == q {
                sum += (1 + p + q + n) as i32;
            }
        }

        sum
    }

    fn create_spf(n: usize) -> Vec<usize> {
        let mut spf = vec![0usize; n+1];
        let mut primes: Vec<usize> = Vec::new();

        if n > 1 { spf[1] = 1; }

        for i in 2..=n {
            if spf[i] == 0 {
                spf[i] = i;
                primes.push(i);
            } 
            for &p in primes.iter() {
                let ip = i * p;
                if ip > n { break; }
                spf[ip] = p;
                if p == spf[i] { break; }
            }
        }

        spf
    }
}