impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let s = format!("{:b}", num).to_string();

        let mut res = 0i32;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '0' {
                res += 2i32.pow(i as u32);
            }
        }

        res
    }
}
