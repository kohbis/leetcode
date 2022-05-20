impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let digits: Vec<char> = num.to_string().chars().collect();

        let k_usize = k as usize;
        let mut count = 0i32;
        for i in 0..=digits.len() - k_usize {
            let s: String = (&digits[i..i + k_usize]).iter().collect();
            match s.parse::<i32>() {
                Ok(i) => {
                    if i != 0 && num % i == 0 {
                        count += 1;
                    }
                }
                Err(e) => {
                    unreachable!()
                }
            }
        }

        count
    }
}
