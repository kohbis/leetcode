use std::collections::HashMap;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut count1 = Solution::letter_count(&word1);
        let mut count2 = Solution::letter_count(&word2);

        for (c, n1) in count1 {
            if let Some(n2) = count2.get(&c) {
                if (n2 - n1).abs() > 3 {
                    return false;
                }

                count2.remove(&c);
            } else {
                if n1 > 3 {
                    return false;
                }
            }
        }

        for (c, n2) in count2 {
            if n2 > 3 {
                return false;
            }
        }

        true
    }

    fn letter_count(s: &str) -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }

        map
    }
}
