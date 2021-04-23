impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut a: Vec<char> = vec![];
        let mut b: Vec<char> = vec![];

        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();

        for i in 0..s1.len() {
            if chars1[i] != chars2[i] {
                a.push(chars1[i]);
                b.push(chars2[i]);

                if a.len() > 2 {
                    return false;
                }
            }
        }

        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}
