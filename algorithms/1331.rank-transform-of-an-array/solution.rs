impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut sorted_scores: Vec<i32> = arr.clone();
        sorted_scores.sort_unstable();
        sorted_scores.dedup();

        let mut rank: HashMap<i32, i32> = sorted_scores
            .iter()
            .enumerate()
            .map(|(idx, &score)| {
                (score, idx as i32 + 1)
            })
            .collect();

        arr
            .iter()
            .map(|score| {
                rank[score]
            })
            .collect()
    }
}
