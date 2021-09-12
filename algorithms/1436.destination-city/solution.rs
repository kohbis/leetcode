use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut departures: HashSet<String> = HashSet::new();
        for path in paths.iter() {
            departures.insert(path[0].clone());
        }

        for path in paths.iter() {
            if !departures.contains(&path[1]) {
                return path[1].clone();
            }
        }

        unreachable!()
    }
}
