use std::collections::BTreeMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
        for m in matches {
            let winner = results.entry(m[0]).or_default();
            winner.0 += 1;
            let loser = results.entry(m[1]).or_default();
            loser.1 += 1;
        }

        let mut answer = vec![vec![], vec![]];
        for (player, result) in results {
            match result.1 {
                0 => answer[0].push(player),
                1 => answer[1].push(player),
                _ => {}
            }
        }

        answer
    }
}
