impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth: i32 = 0;
        let move_to_parent_folder: &str = "../";
        let remain_in_same_folder: &str = "./";

        for log in logs {
            if move_to_parent_folder == log {
                if depth > 0 {
                    depth -= 1
                }
            } else if remain_in_same_folder != log {
                depth += 1
            }
        }

        depth
    }
}
