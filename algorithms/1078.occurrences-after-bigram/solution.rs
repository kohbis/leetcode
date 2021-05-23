impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words = text.split_whitespace().collect::<Vec<&str>>();
        let mut res: Vec<String> = vec![];

        for i in 2..words.len() {
            if words[i - 2] == &first && words[i - 1] == &second {
                res.push(words[i].to_string());
            }
        }

        res
    }
}
