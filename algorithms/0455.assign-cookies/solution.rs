impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort_unstable();
        s.sort_unstable();

        let mut res = 0;
        let (mut child_idx, mut cookie_idx) = (0, 0);
        while child_idx < g.len() && cookie_idx < s.len() {
            if g[child_idx] <= s[cookie_idx] {
                res += 1;
                child_idx += 1;
            }
            cookie_idx += 1;
        }

        res
    }
}
