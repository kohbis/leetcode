impl Solution {
    pub fn reformat_date(date: String) -> String {
        let mut date = date.split_whitespace().collect::<Vec<&str>>();

        let months = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let month = format!("{:02}", (months.iter().position(|&m| m == date[1]).unwrap() + 1));

        let day = format!("{:02}", date[0].replace(&['d', 'h', 'n', 'r', 's', 't'][..], "").parse::<u8>().unwrap());

        [date[2], &month, &day].join("-")
    }
}
