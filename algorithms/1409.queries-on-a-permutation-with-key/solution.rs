impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = (1..=m).collect::<Vec<i32>>();

        let mut res: Vec<i32> = vec![];
        for n in queries {
            let idx = p.iter().position(|&x| x == n).unwrap();
            res.push(idx as i32);

            p.remove(idx);
            p.insert(0, n);
        }

        res
    }
}
