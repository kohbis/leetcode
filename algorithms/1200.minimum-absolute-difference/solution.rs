impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut min_diff: i32 = -1;
        let mut arr: Vec<i32> = arr.clone();

        arr.sort();

        for w in arr.windows(2) {
            let (a, b) = (w[0], w[1]);
            let diff = b - a;

            if min_diff < 0 {
                min_diff = diff;
            }

            if min_diff > diff {
                min_diff = diff;
                res.clear();
                res.push(vec![a, b]);
            } else if min_diff == diff {
                res.push(vec![a, b]);
            }
        }

        res.to_vec()
    }
}
