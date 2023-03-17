impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let mut res: i32 = 0;
        for word in &words[(left as usize)..=(right as usize)] {
            let chars: Vec<char> = word.chars().collect();
            if vowels.contains(&chars[0]) && vowels.contains(&chars[chars.len() - 1]) {
                res += 1;
            }
        }
        res
    }
}
