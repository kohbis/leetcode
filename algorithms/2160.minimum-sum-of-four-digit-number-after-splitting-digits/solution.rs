impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut nums: Vec<char> = num.to_string().chars().collect();
        nums.sort_unstable();

        let mut ans = 0i32;
        for (i, c) in nums.iter().enumerate() {
            let n = c.to_string().parse::<i32>().unwrap();
            match i {
                0 | 1 => ans += n * 10,
                _ => ans += n,
            }
        }

        ans
    }
}
