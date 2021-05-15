impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        words
            .iter()
            .map(|s| Solution::special_sort(&s))
            .collect::<HashSet<String>>()
            .len() as i32
    }

    fn special_sort(s: &String) -> String {
        let mut odd: Vec<char> = s.chars().step_by(2).collect();
        let mut even: Vec<char> = s.chars().skip(1).step_by(2).collect();

        odd.sort_unstable();
        even.sort_unstable();

        [
            odd.iter().collect::<String>(),
            even.iter().collect::<String>(),
        ]
        .concat()
    }
}
