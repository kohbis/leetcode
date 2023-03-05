impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut carry: i32 = 1; // Plus one
        for (i, d) in digits.iter().rev().enumerate() {
            let n = d + carry;
            res.insert(0, n % 10);
            carry = n / 10;
        }
        if carry > 0 {
            res.insert(0, carry);
        }
        res
    }
}
