impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let s_ab = (points[0][0] - points[1][0]) * (points[0][1] - points[2][1]);
        let s_ac = (points[0][0] - points[2][0]) * (points[0][1] - points[1][1]);
        s_ab != s_ac
    }
}
