impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut v_res: Vec<char> = s.clone().chars().collect();
        let v_s: Vec<char> = s.chars().collect();
        for (i, &n) in indices.iter().enumerate() {
            v_res[n as usize] = v_s[i];
        }
        v_res.iter().collect()
    }
}
