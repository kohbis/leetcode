impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut stack: Vec<char> = s.chars().filter(|x| vowels.contains(x)).collect();

        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            if vowels.contains(&c) {
                res.push(stack.pop().unwrap());
            } else {
                res.push(c);
            }
        }

        res.iter().collect()
    }
}
