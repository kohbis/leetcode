impl Solution {
    pub fn maximum_time(time: String) -> String {
        let digits: Vec<char> = time.chars().collect();
        let mut res: Vec<char> = vec![];

        // "*x:xx"
        if digits[0] != '?' {
            res.push(digits[0])
        } else {
            if digits[1] == '?' || (digits[1] as i32 - 48) < 4 {
                res.push('2')
            } else {
                res.push('1')
            }
        }

        // "x*:xx"
        if digits[1] != '?' {
            res.push(digits[1])
        } else {
            if res[0] == '2' {
                res.push('3')
            } else {
                res.push('9')
            }
        }

        // "xx*xx"
        res.push(':');

        // "xx:*x"
        if digits[3] != '?' {
            res.push(digits[3])
        } else {
            res.push('5')
        }

        // "xx:x*"
        if digits[4] != '?' {
            res.push(digits[4])
        } else {
            res.push('9')
        }

        // collect
        res.into_iter().collect()
    }
}
