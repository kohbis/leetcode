impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0i32;

        for n in s.split(' ').filter_map(|x| x.parse::<i32>().ok()) {
            match prev < n {
                true => prev = n,
                false => return false,
            }
        }

        true
    }
}
