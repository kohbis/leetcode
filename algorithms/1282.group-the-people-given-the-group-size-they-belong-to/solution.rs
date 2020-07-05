use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups = vec![];
        let mut tmp: HashMap<i32, Vec<i32>> = HashMap::new();
        for (id, group_size) in group_sizes.into_iter().enumerate() {
            let group = tmp.entry(group_size).or_insert(vec![]);
            group.push(id as i32);
            if group.len() == group_size as usize {
                groups.push(group.clone());
                tmp.remove(&group_size);
            }
        }
        groups
    }
}
