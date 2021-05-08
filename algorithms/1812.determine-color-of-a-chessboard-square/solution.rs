impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let y = coordinates.chars().nth(1).unwrap() as usize - 1;
        let x = coordinates.chars().nth(0).unwrap();
        let pos = (b'a'..=b'h')
            .map(|c| c as char)
            .collect::<Vec<_>>()
            .iter()
            .position(|&r| r == x)
            .unwrap();

        (y + pos) % 2 != 0
    }
}
