impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let mut res: Vec<String> = vec![];
        for part in date.split("-") {
            res.push(format!("{:b}", part.parse::<i32>().unwrap()));
        }
        res.join("-")
    }
}
