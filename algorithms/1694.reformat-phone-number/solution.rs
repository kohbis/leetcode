impl Solution {
    pub fn reformat_number(number: String) -> String {
        use std::collections::VecDeque;

        let mut parts: Vec<String> = vec![];
        let mut nums: VecDeque<char> = number.clone().replace("-", "").replace(" ", "").chars().collect();

        let mut three_digits = nums.len() / 3;
        let mut two_digits = match nums.len() % 3 {
            1 => {
                three_digits -= 1;
                2
            },
            2 => 1,
            _ => 0,
        };

        for _ in 0..three_digits {
            let mut part: String = "".to_string();
            for _ in 0..3 {
                part.push_str(&(nums.pop_front().unwrap().to_string()));
            }
            parts.push(part);
        }

        for _ in 0..two_digits {
            let mut part: String = "".to_string();
            for _ in 0..2 {
                part.push_str(&(nums.pop_front().unwrap().to_string()));
            }
            parts.push(part);
        }

        parts.join("-")
    }
})
