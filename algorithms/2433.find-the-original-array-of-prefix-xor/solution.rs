impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        res.push(pref[0]);
        for i in 1..pref.len() {
            res.push(pref[i - 1] ^ pref[i]);
        }
        res
    }
}
