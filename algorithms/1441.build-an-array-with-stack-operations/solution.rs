impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut arr: Vec<String> = vec![];
        let last = *target.last().unwrap();

        for i in 1..=last {
            arr.push("Push".to_string());
            if !target.contains(&i) {
                arr.push("Pop".to_string());
            }
        }

        arr
    }
}
