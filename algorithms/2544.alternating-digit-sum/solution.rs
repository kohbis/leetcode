impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n;
        let mut nums: Vec<i32> = vec![];
        while n > 0 {
            let r = n % 10;
            nums.push(r);
            n /= 10;
        }

        let mut res: i32 = 0;
        for (i, num) in nums.into_iter().rev().enumerate() {
            res += if i % 2 == 0 { num } else { num * -1 }
        }
        res
    }
}
