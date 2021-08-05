use std::cmp::min;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut dist: Vec<i32> = vec![0; num_people as usize];

        let mut remains = candies;
        let mut takes = 0;
        while remains > 0 {
            dist[(takes % num_people) as usize] += min(remains, takes + 1);
            takes += 1;
            remains -= takes;
        }

        dist
    }
}
