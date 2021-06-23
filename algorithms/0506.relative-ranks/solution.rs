impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut idxs: Vec<(usize, i32)> =
            score.into_iter().enumerate().map(|(i, s)| (i, s)).collect();
        idxs.sort_by(|a, b| b.1.cmp(&a.1));

        let mut res: Vec<String> = vec!["".to_string(); idxs.len()];
        for (rank, idx) in idxs.iter().enumerate() {
            match rank {
                0 => res[idx.0] = "Gold Medal".to_string(),
                1 => res[idx.0] = "Silver Medal".to_string(),
                2 => res[idx.0] = "Bronze Medal".to_string(),
                _ => res[idx.0] = (rank + 1).to_string(),
            }
        }

        res
    }
}
