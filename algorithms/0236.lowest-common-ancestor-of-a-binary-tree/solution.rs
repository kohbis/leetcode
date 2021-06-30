impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut res = letters[0];

        for i in 0..letters.len() {
            if target < letters[i] {
                res = letters[i];
                break;
            }
        }

        res
    }
}
