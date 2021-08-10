impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = vec![];

        let mut last = ' ';
        let mut second_last = ' ';

        for c in s.chars() {
            if c != last || c != second_last {
                res.push(c);
            }

            second_last = last;
            last = c;
        }

        res.iter().collect::<String>()
    }
}
