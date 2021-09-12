impl Solution {
    pub fn sort_string(s: String) -> String {
        // result
        let mut res: Vec<char> = Vec::new();
        // sorted charactors
        let mut seeds: Vec<char> = s.chars().collect();
        seeds.sort();
        // reverse flag
        let mut rev: bool = false;

        while seeds.len() > 0 {
            let mut additions = seeds.clone();
            additions.dedup();

            if rev {
                additions = additions.into_iter().rev().collect();
            };

            for c in additions.into_iter() {
                res.push(c);

                let index = seeds.iter().position(|x| *x == c).unwrap();
                seeds.remove(index);
            }

            // toggle
            rev = !rev;
        }

        res.iter().collect()
    }
}
