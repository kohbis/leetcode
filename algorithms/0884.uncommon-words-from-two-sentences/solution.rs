impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut map: HashMap<&str, i32> = HashMap::new();

        for sentence in vec![&a, &b] {
            for word in sentence.split_whitespace() {
                let value = map.entry(word).or_insert(0);
                *value += 1;
            }
        }

        let mut res: Vec<String> = Vec::new();
        for (k, v) in map {
            if v == 1 {
                res.push(k.to_string());
            }
        }
        res
    }
}
