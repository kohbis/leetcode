impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0i32;

        for op in operations {
            match op.as_ref() {
                "++X" | "X++" => res += 1,
                "--X" | "X--" => res -= 1,
                _ => {}
            }
        }

        res
    }
}
