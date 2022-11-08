impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(last) = stack.last() {
                if (last.is_uppercase() && last.to_ascii_lowercase() == c)
                    || (last.is_lowercase() && last.to_ascii_uppercase() == c)
                {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.iter().collect()
    }
}
