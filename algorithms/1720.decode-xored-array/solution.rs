impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![first];

        for i in 0..encoded.len() {
            res.push(res[i] ^ encoded[i])
        }

        res
    }
}
