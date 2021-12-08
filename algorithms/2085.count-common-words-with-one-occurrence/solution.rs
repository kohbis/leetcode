use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let count1 = Solution::counter(&words1);
        let count2 = Solution::counter(&words2);

        let mut res = 0i32;
        for (s, n) in count1 {
            if n == 1 {
                if let Some(v) = count2.get(s) {
                    if v == &1 {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    fn counter(words: &Vec<String>) -> HashMap<&str, i32> {
        let mut count: HashMap<&str, i32> = HashMap::new();

        for word in words {
            let entry = count.entry(word).or_insert(0);
            *entry += 1;
        }

        count
    }
}
