impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let keys = vec!["type", "color", "name"];
        let index = keys.iter().position(|&k| rule_key == k).unwrap();

        let mut count = 0;
        for item in items {
            if rule_value == item[index] {
                count += 1
            }
        }
        count
    }
}
