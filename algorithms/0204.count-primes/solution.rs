impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let n_usize = n as usize;
        let mut primes: Vec<bool> = vec![true; n_usize];
        primes[0] = false;
        primes[1] = false;

        for i in 2..n_usize {
            if primes[i] {
                for j in (i * 2..n_usize).step_by(i) {
                    primes[j] = false;
                }
            }
        }

        primes.into_iter().filter(|&x| x).count() as i32
    }
}
