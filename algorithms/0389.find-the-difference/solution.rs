impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut letters: Vec<char> = t.chars().collect();

        for c in s.chars() {
            let i = letters.iter().position(|&x| x == c).unwrap();
            letters.remove(i);
        }

        letters[0]
    }
}
