impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        num.to_string()
            .bytes()
            .filter(|x| num % (x - 48) as i32 == 0)
            .count() as i32
    }
}
