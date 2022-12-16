impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        use std::collections::HashMap;

        let directions: Vec<char> = vec!['L', 'R', 'U', 'D'];
        let sequences: Vec<char> = moves.chars().collect();

        let mut map: HashMap<char, i32> = HashMap::new();

        for d in directions.iter() {
            let count = sequences.iter().filter(|&x| x == d).count() as i32;
            map.insert(*d, count);
        }

        map.get(&'L') == map.get(&'R') && map.get(&'U') == map.get(&'D')
    }
}
