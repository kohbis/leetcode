impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut dists: Vec<Vec<i32>> = vec![];

        for (idx, point) in points.iter().enumerate() {
            let (px, py) = (point[0], point[1]);
            if x == px || y == py {
                dists.push(vec![(x - px).abs() + (y - py).abs(), idx as i32])
            }
        }

        if dists.len() == 0 {
            -1
        } else {
            dists.sort();
            dists[0][1]
        }
    }
}
