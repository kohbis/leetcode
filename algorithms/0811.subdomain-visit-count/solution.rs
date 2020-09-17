impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut map: HashMap<String, i32> = HashMap::new();

        for cpdomain in cpdomains {
            let pair = (cpdomain.split_whitespace().collect::<Vec<&str>>());
            let visits = pair[0].parse::<i32>().unwrap();
            let domains = pair[1].split(".").collect::<Vec<&str>>();

            let l = domains.len();

            for i in 0..l {
                let level = &domains[i..l].join(".");

                let total_visits = map.entry(level.to_string()).or_insert(0);
                *total_visits += visits;
            }
        }

        let mut res = Vec::with_capacity(map.len());
        for (k, v) in map {
            res.push(v.to_string() + " " + &k);
        }

        res
    }
}
