impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut letters: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();

        s.chars()
            .map(|c| {
                if c.is_alphabetic() { letters.pop().unwrap() } else { c }
            })
            .collect::<String>()
    }
}
