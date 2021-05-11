impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            if res.len() > 0 && res[res.len() - 1] == c {
                res.pop();
                continue;
            }
            res.push(c);
        }
        res.iter().collect::<String>()
    }
}
