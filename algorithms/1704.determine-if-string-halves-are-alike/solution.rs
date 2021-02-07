impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let mid = s.len() / 2;
        let mut a_vowels = 0;
        let mut b_vowels = 0;

        for (i, c) in s.chars().enumerate() {
            if vowels.contains(&c) {
                if i < mid {
                    a_vowels += 1
                } else {
                    b_vowels += 1
                }
            }
        }

        a_vowels == b_vowels
    }
}
