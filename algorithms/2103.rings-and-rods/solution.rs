use std::collections::HashMap;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods: HashMap<char, Vec<char>> = HashMap::new();

        for chunk in rings.chars().collect::<Vec<_>>().chunks(2) {
            let (color, ring) = (chunk[0], chunk[1]);

            if let Some(rod) = rods.get_mut(&ring) {
                if !rod.contains(&color) {
                    rod.push(color);
                }
            } else {
                rods.insert(ring, vec![color]);
            }
        }

        let mut res = 0i32;
        for (_, v) in rods {
            if v.len() == 3 {
                res += 1;
            }
        }

        res
    }
}
