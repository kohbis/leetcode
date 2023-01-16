impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let chars: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            let mut curr_pos = start_pos.clone();
            let mut count = 0i32;

            for j in i..s.len() {
                match chars[j] {
                    'L' => curr_pos[1] -= 1,
                    'R' => curr_pos[1] += 1,
                    'U' => curr_pos[0] -= 1,
                    'D' => curr_pos[0] += 1,
                    _ => {}
                }

                if curr_pos[0] < 0 || n <= curr_pos[0] || curr_pos[1] < 0 || n <= curr_pos[1] {
                    break;
                }

                count += 1;
            }

            res.push(count);
        }

        res
    }
}
