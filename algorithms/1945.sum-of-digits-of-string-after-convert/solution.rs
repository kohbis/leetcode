impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum = 0;

        for c in s.chars() {
            let mut n = (c as usize - 'a' as usize + 1) as i32;
            while n != 0 {
                sum += n % 10;
                n /= 10;
            }
        }

        for _ in 1..k {
            let mut curr = sum;
            sum = 0;

            while curr != 0 {
                sum += curr % 10;
                curr /= 10;
            }
        }

        sum
    }
}
