impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target {
            return true;
        }

        let l = mat.len();
        let mut rot = mat.clone();

        for _ in 0..3 {
            let mut buf = rot.clone();

            for i in 0..l {
                for j in 0..l {
                    rot[i][j] = buf[l - j - 1][i];
                }
            }

            if target == rot {
                return true;
            }
        }

        false
    }
}
