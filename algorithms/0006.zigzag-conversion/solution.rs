impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let s_len = s.len();
        let num_rows = num_rows as usize;

        if num_rows == 1 || s_len <= num_rows {
            return s;
        }

        let mut rows = vec![vec![]; num_rows];
        let mut row = 0;
        let mut i = 1i32;
        for c in s.chars() {
            rows[row].push(c);

            if row == 0 {
                i = 1;
            } else if row == num_rows - 1 {
                i = -1;
            }

            row = (row as i32 + i) as usize;
        }

        return rows.iter().fold("".to_string(), |s, t| {
            format!("{}{}", s, t.iter().collect::<String>())
        });
    }
}
