impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Solution::string_builder(&s) == Solution::string_builder(&t)
    }

    fn string_builder(text: &str) -> String {
        let mut arr: Vec<char> = vec![];

        for c in text.chars() {
            if c == '#' {
                arr.pop();
            } else {
                arr.push(c)
            }
        }

        arr.iter().collect()
    }
}
