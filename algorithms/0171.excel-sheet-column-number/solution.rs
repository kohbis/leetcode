impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column_number = 0;

        for c in column_title.chars() {
            let n = c as usize - 'A' as usize + 1;
            column_number = column_number * 26 + n;
        }

        column_number as i32
    }
}
