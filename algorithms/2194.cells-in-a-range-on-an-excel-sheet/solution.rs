impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let bs = s.as_bytes();

        let mut list: Vec<String> = vec![];
        for row in bs[0]..=bs[3] {
            for col in bs[1]..=bs[4] {
                list.push(format!("{}{}", row as char, col as char));
            }
        }

        list
    }
}
