impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<i32, bool> = HashMap::new();

        for i in arr {
            if map.contains_key(&(i * 2)) {
                if map[&(i * 2)] {
                    return true;
                }
            }

            if (i % 2 == 0) && map.contains_key(&(i / 2)) {
                if map[&(i / 2)] {
                    return true;
                }
            }

            map.insert(i, true);
        }

        // default
        false
    }
}
