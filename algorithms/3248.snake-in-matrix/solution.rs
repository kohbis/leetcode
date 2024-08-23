impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        for command in commands {
            match command.as_str() {
                "UP" => i -= 1,
                "DOWN" => i += 1,
                "LEFT" => j -= 1,
                "RIGHT" => j += 1,
                _ => (),
            }
            i = i.max(0).min(n - 1);
            j = j.max(0).min(n - 1);
        }
        i * n + j
    }
}
