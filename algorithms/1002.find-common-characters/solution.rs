impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut count_map: HashMap<char, usize> = HashMap::new();

        for c in a[0].chars() {
            let count = count_map.entry(c).or_insert(0);
            *count += 1;
        }

        for i in 1..a.len() {
            let word = &a[i];
            for (key, value) in count_map.iter_mut() {
                let count = word.chars()
                                .collect::<Vec<char>>()
                                .iter()
                                .filter(|&x| x == key)
                                .count();

                *value = std::cmp::min(*value, count);
            }
        }

        count_map
            .iter()
            .flat_map(|(key, value)| std::iter::repeat(key).take(*value))
            .map(|c| c.to_string())
            .collect()
    }
}
