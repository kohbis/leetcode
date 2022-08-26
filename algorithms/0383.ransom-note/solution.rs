impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut mag_chars: Vec<char> = magazine.chars().collect();
        for c in ransom_note.chars() {
            if let Some(idx) = mag_chars.iter().position(|x| x == &c) {
                mag_chars.remove(idx);
            } else {
                return false;
            }
        }

        true
    }
}
