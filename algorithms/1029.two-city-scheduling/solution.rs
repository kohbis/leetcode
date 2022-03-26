impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_unstable_by(|a, b| (b[0] - b[1]).cmp(&(a[0] - a[1])));

        let mut ans = 0i32;
        let mid = costs.len() / 2;
        for i in 0..mid {
            ans += costs[i][1] + costs[i + mid][0];
        }

        ans
    }
}
