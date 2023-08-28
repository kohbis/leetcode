impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut point, mut underscore) = (0, 0);
        for c in moves.chars() {
            match c {
                'L' => point -= 1,
                'R' => point += 1,
                _ => underscore += 1,
            }
        }
        if 0 < point {
            point + underscore
        } else {
            (point - underscore).abs()
        }
    }
}
