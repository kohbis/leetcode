impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut words = words.clone();
        words.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    res.push(words[i].clone());
                    break;
                }
            }
        }

        res
    }
}
