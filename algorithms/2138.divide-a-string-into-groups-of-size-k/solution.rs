impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k_usize = k as usize;
        let mut res: Vec<String> = vec![];
        let mut chs: Vec<Vec<char>> = s
            .chars()
            .collect::<Vec<_>>()
            .chunks(k_usize)
            .map(|x| x.to_vec())
            .collect();

        for ch in chs {
            let mut s = ch.iter().collect::<String>();
            if s.len() == k_usize {
                res.push(s);
            } else {
                res.push(format!(
                    "{}{}",
                    s,
                    fill.to_string().repeat(k_usize - s.len())
                ));
            }
        }

        res
    }
}
