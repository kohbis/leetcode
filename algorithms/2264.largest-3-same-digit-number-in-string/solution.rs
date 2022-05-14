impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        for i in (0..=9).rev() {
            let p = i.to_string().repeat(3);
            if num.contains(&p) {
                return p;
            }
        }

        "".to_string()
    }
}
