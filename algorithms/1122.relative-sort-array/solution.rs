impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut first: Vec<i32> = vec![];

        for n1 in arr1 {
            let value = map.entry(n1).or_insert(0);
            *value += 1;
        }
        for n2 in arr2 {
            let mut sub = vec![n2; map[&n2]];
            first.append(&mut sub);
            map.remove(&n2);
        }

        let mut second: Vec<i32> = vec![];
        for (k, v) in map {
            let mut sub = vec![k; v];
            second.append(&mut sub);
        }

        second.sort_unstable();
        first.append(&mut second);
        first
    }
}
