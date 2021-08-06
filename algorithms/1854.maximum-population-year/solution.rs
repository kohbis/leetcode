use std::collections::HashMap;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut max_pop = 0;
        let mut max_pop_year = logs[0][0];
        for log in logs {
            for year in log[0]..log[1] {
                *map.entry(year).or_insert(0) += 1;

                if max_pop < map[&year] {
                    max_pop = map[&year];
                    max_pop_year = year;
                } else if max_pop == map[&year] {
                    if year < max_pop_year {
                        max_pop_year = year;
                    }
                }
            }
        }

        max_pop_year
    }
}
