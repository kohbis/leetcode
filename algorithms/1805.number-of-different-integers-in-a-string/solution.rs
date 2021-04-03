impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut nums: Vec<&str> = (&word)
            .split(char::is_alphabetic)
            .filter(|x| !x.is_empty())
            .map(|n| n.trim_start_matches('0'))
            .collect();

        nums.sort_unstable();
        nums.dedup();

        nums.len() as i32
    }
}
