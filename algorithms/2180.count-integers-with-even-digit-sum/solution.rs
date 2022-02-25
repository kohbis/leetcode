impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut ans = 0i32;

        for i in 1..=num {
            if Solution::digit_sum(i) % 2 == 0 {
                ans += 1
            }
        }

        ans
    }

    fn digit_sum(x: i32) -> i32 {
        let digit_chars: Vec<char> = x.to_string().chars().collect();

        let mut sum = 0i32;
        for c in digit_chars {
            sum += c as i32 - 48;
        }

        sum
    }
}
