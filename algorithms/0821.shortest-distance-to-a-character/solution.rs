impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut res: Vec<i32> = Vec::with_capacity(s.len());
        let mut index_queue: VecDeque<i32> = VecDeque::new();
        let mut last_c_idx: i32 = -1;

        for (i, x) in s.chars().enumerate() {
            let i = i as i32;
            if x == c {
                while index_queue.len() > 0 {
                    if last_c_idx < 0 {
                        last_c_idx = i;
                    }
                    let first = index_queue.pop_front().unwrap();
                    res.push(std::cmp::min(i - first, (last_c_idx - first).abs()));
                }
                res.push(0);
                last_c_idx = i
            } else {
                index_queue.push_back(i)
            }
        }

        while index_queue.len() > 0 {
            let first = index_queue.pop_front().unwrap();
            res.push((last_c_idx - first).abs());
        }

        res
    }
}
