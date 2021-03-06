impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count1 = 0 as i32;
        let mut count2 = 0 as i32;

        let bits: Vec<char> = s.chars().collect();
        for (i, &c) in bits.iter().enumerate() {
            if i % 2 == 0 {
                if c == '0' {
                    count1 += 1
                } else {
                    count2 += 1
                }
            } else {
                if c == '1' {
                    count1 += 1
                } else {
                    count2 += 1
                }
            }
        }

        if count1 < count2 {
            count1
        } else {
            count2
        }
    }
}
