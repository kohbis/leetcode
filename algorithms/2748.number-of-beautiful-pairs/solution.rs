impl Solution {
    fn first_digit(num: i32) -> i32 {
        let mut n = num;
        while n >= 10 {
            n /= 10;
        }
        n
    }

    fn last_digit(num: i32) -> i32 {
        num % 10
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let mut res = a.min(b);
        while res > 0 {
            if a % res == 0 && b % res == 0 {
                break;
            }
            res -= 1;
        }
        res
    }

    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let first = Self::first_digit(nums[i]);
                let last = Self::last_digit(nums[j]);
                println!("{}, {}", first, last);
                if Self::gcd(first, last) == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}
