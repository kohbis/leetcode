impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() == 0 {
            return 1;
        }
        let mut people: Vec<i32> = vec![0; n as usize];
        for t in trust.iter() {
            people[t[0] as usize - 1] -= 1;
            people[t[1] as usize - 1] += 1;
        }
        for (idx, p) in people.iter().enumerate() {
            if *p == n - 1 {
                return idx as i32 + 1;
            }
        }
        -1
    }
}
