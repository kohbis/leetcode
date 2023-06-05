impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut is_straight = true;

        if coordinates.len() <= 2 {
            return is_straight;
        }

        let (x1, y1) = (coordinates[0][0], coordinates[0][1]);
        let (x2, y2) = (coordinates[1][0], coordinates[1][1]);

        for i in 2..coordinates.len() {
            let (x, y) = (coordinates[i][0], coordinates[i][1]);
            if (x1 - x) * (y2 - y1) != (y1 - y) * (x2 - x1) {
                is_straight = false;
                break;
            }
        }

        is_straight
    }
}
