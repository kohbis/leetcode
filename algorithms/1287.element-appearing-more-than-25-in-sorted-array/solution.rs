impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let quarter: f32 = arr.len() as f32 * 0.25;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in arr {
            let entry = map.entry(n).or_insert(0);
            *entry += 1;
            
            if *entry as f32 > quarter {
                return n
            }
        }

        unreachable!()
    }
}
