use std::collections::BTreeMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut count: BTreeMap<i32, i32> = BTreeMap::new();
        for item in items1.iter().chain(items2.iter()) {
            *count.entry(item[0]).or_insert(0) += item[1];
        }
        count
            .into_iter()
            .map(|(value, weight)| vec![value, weight])
            .collect()
    }
}
