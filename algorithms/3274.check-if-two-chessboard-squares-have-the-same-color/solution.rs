impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let mut c1 = coordinate1.chars();
        let mut c2 = coordinate2.chars();
        let x1 = c1.next().unwrap() as i32 - 'a' as i32 + 1;
        let y1 = c1.next().unwrap() as i32 - '1' as i32;
        let x2 = c2.next().unwrap() as i32 - 'a' as i32 + 1;
        let y2 = c2.next().unwrap() as i32 - '1' as i32;

        (x1 + y1) % 2 == (x2 + y2) % 2
    }
}
